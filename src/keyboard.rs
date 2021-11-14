// This file is part of toy_xcb and is released under the terms
// of the MIT license. See included LICENSE.txt file.

use event::Event;
use key;
use xkbcommon::xkb;

use xcb;

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::io::{stderr, Write};
use std::mem;

pub struct Keyboard {
    context: xkb::Context,
    device_id: i32,
    keymap: xkb::Keymap,
    state: RefCell<xkb::State>,
    keysym_map: HashMap<u32, key::Sym>,
    keycode_table: [key::Code; 256],
    mods: Cell<u8>,
}

impl Keyboard {
    pub fn new(connection: &xcb::Connection) -> (Keyboard, u8, u8) {
        connection.prefetch_extension_data(xcb::xkb::id());

        let (first_ev, first_er) = match connection.get_extension_data(xcb::xkb::id()) {
            Some(r) => (r.first_event(), r.first_error()),
            None => {
                panic!("could not get xkb extension data");
            }
        };

        {
            let cookie = xcb::xkb::use_extension(
                &connection,
                xkb::x11::MIN_MAJOR_XKB_VERSION,
                xkb::x11::MIN_MINOR_XKB_VERSION,
            );
            match cookie.get_reply() {
                Ok(r) => {
                    if !r.supported() {
                        panic!(
                            "required xcb-xkb-{}-{} is not supported",
                            xkb::x11::MIN_MAJOR_XKB_VERSION,
                            xkb::x11::MIN_MINOR_XKB_VERSION
                        );
                    }
                }
                Err(_) => {
                    panic!("could not check if xkb is supported");
                }
            }
        }

        {
            let map_parts = xcb::xkb::MAP_PART_KEY_TYPES
                | xcb::xkb::MAP_PART_KEY_SYMS
                | xcb::xkb::MAP_PART_MODIFIER_MAP
                | xcb::xkb::MAP_PART_EXPLICIT_COMPONENTS
                | xcb::xkb::MAP_PART_KEY_ACTIONS
                | xcb::xkb::MAP_PART_KEY_BEHAVIORS
                | xcb::xkb::MAP_PART_VIRTUAL_MODS
                | xcb::xkb::MAP_PART_VIRTUAL_MOD_MAP;

            let events = xcb::xkb::EVENT_TYPE_NEW_KEYBOARD_NOTIFY
                | xcb::xkb::EVENT_TYPE_MAP_NOTIFY
                | xcb::xkb::EVENT_TYPE_STATE_NOTIFY;

            let cookie = xcb::xkb::select_events_checked(
                &connection,
                xcb::xkb::ID_USE_CORE_KBD as u16,
                events as u16,
                0,
                events as u16,
                map_parts as u16,
                map_parts as u16,
                None,
            );

            cookie
                .request_check()
                .expect("failed to select notify events from xcb xkb");
        }

        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let device_id = xkb::x11::get_core_keyboard_device_id(&connection);
        let keymap = xkb::x11::keymap_new_from_device(
            &context,
            &connection,
            device_id,
            xkb::KEYMAP_COMPILE_NO_FLAGS,
        );
        let state = xkb::x11::state_new_from_device(&keymap, &connection, device_id);

        let kbd = Keyboard {
            context: context,
            device_id: device_id,
            keymap: keymap,
            state: RefCell::new(state),
            keysym_map: build_keysym_map(),
            keycode_table: build_keycode_table(),
            mods: Cell::new(0),
        };

        (kbd, first_ev, first_er)
    }

    pub fn make_key_event(&self, xcb_ev: &xcb::KeyPressEvent, press: bool) -> Event {
        let xcode = xcb_ev.detail() as xkb::Keycode;
        let xsym = self.state.borrow().key_get_one_sym(xcode);
        let pressed = (xcb_ev.response_type() & !0x80) == xcb::KEY_PRESS;

        let code = self.get_keycode(xcode);
        let mut mod_mask: u8 = 0;
        match code {
            key::Code::LeftCtrl => {
                mod_mask |= key::MODS_LEFT_CTRL;
            }
            key::Code::LeftShift => {
                mod_mask |= key::MODS_LEFT_SHIFT;
            }
            key::Code::LeftAlt => {
                mod_mask |= key::MODS_LEFT_ALT;
            }
            key::Code::LeftSuper => {
                mod_mask |= key::MODS_LEFT_SUPER;
            }
            key::Code::RightCtrl => {
                mod_mask |= key::MODS_RIGHT_CTRL;
            }
            key::Code::RightShift => {
                mod_mask |= key::MODS_RIGHT_SHIFT;
            }
            key::Code::RightAlt => {
                mod_mask |= key::MODS_RIGHT_ALT;
            }
            key::Code::RightSuper => {
                mod_mask |= key::MODS_RIGHT_SUPER;
            }
            _ => {}
        }

        if mod_mask != 0 {
            let mut mods = self.mods.get();
            if pressed {
                mods |= mod_mask;
            } else {
                mods &= !mod_mask;
            }
            self.mods.set(mods);
        }

        if press {
            Event::KeyPress(
                self.get_keysym(xsym),
                code,
                self.state.borrow().key_get_utf8(xcode),
            )
        } else {
            Event::KeyRelease(self.get_keysym(xsym), code, String::new())
        }
    }

    pub fn get_mods(&self) -> key::Mods {
        key::Mods::new(self.mods.get())
    }

    // for convenience, this fn takes &self, not &mut self
    pub fn update_state(&self, ev: &xcb::xkb::StateNotifyEvent) {
        self.state.borrow_mut().update_mask(
            ev.base_mods() as xkb::ModMask,
            ev.latched_mods() as xkb::ModMask,
            ev.locked_mods() as xkb::ModMask,
            ev.base_group() as xkb::LayoutIndex,
            ev.latched_group() as xkb::LayoutIndex,
            ev.locked_group() as xkb::LayoutIndex,
        );
    }

    pub fn get_device_id(&self) -> i32 {
        self.device_id
    }

    fn mod_active(&self, name: &str) -> bool {
        let ind = self.keymap.mod_get_index(&name);
        self.state
            .borrow()
            .mod_index_is_active(ind, xkb::STATE_MODS_DEPRESSED)
    }

    fn get_keycode(&self, xcode: xkb::Keycode) -> key::Code {
        let xcode = xcode as usize;
        if xcode >= self.keycode_table.len() {
            writeln!(&mut stderr(), "keycode 0x{:x} is out of bounds", xcode);
            return key::Code::Unknown;
        }
        self.keycode_table[xcode]
    }

    fn get_keysym(&self, xsym: xkb::Keysym) -> key::Sym {
        if xsym >= 0x20 && xsym < 0x80 {
            let mut xsym = xsym;
            if xsym >= 0x61 && xsym <= 0x7a {
                xsym &= !(key::SYM_LATIN1_SMALL_MASK as u32);
            }
            unsafe { mem::transmute(xsym) }
        } else if xsym >= xkb::KEY_F1 && xsym <= xkb::KEY_F24 {
            unsafe { mem::transmute((key::Sym::F1 as u32) + (xsym - xkb::KEY_F1)) }
        } else if let Some(k) = self.keysym_map.get(&xsym) {
            *k
        } else {
            key::Sym::Unknown
        }
    }
}

fn build_keycode_table() -> [key::Code; 256] {
    [
        // 0x00     0
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Escape,
        key::Code::N1,
        key::Code::N2,
        key::Code::N3,
        key::Code::N4,
        key::Code::N5,
        key::Code::N6,
        // 0x10     16
        key::Code::N7,
        key::Code::N8,
        key::Code::N9,
        key::Code::N0,
        key::Code::Minus,
        key::Code::Equals,
        key::Code::Backspace,
        key::Code::Tab,
        key::Code::Q,
        key::Code::W,
        key::Code::E,
        key::Code::R,
        key::Code::T,
        key::Code::Y,
        key::Code::U,
        key::Code::I,
        // 0x20     32
        key::Code::O,
        key::Code::P,
        key::Code::LeftBracket,
        key::Code::RightBracket,
        key::Code::Enter,
        key::Code::LeftCtrl,
        key::Code::A,
        key::Code::S,
        key::Code::D,
        key::Code::F,
        key::Code::G,
        key::Code::H,
        key::Code::J,
        key::Code::K,
        key::Code::L,
        key::Code::Semicolon,
        // 0x30     48
        key::Code::Quote,
        key::Code::Grave,
        key::Code::LeftShift,
        key::Code::UK_Hash,
        key::Code::Z,
        key::Code::X,
        key::Code::C,
        key::Code::V,
        key::Code::B,
        key::Code::N,
        key::Code::M,
        key::Code::Comma,
        key::Code::Period,
        key::Code::Slash,
        key::Code::RightShift,
        key::Code::KP_Multiply,
        // 0x40     64
        key::Code::LeftAlt,
        key::Code::Space,
        key::Code::CapsLock,
        key::Code::F1,
        key::Code::F2,
        key::Code::F3,
        key::Code::F4,
        key::Code::F5,
        key::Code::F6,
        key::Code::F7,
        key::Code::F8,
        key::Code::F9,
        key::Code::F10,
        key::Code::KP_NumLock,
        key::Code::ScrollLock,
        key::Code::KP_7,
        // 0x50     80
        key::Code::KP_8,
        key::Code::KP_9,
        key::Code::KP_Subtract,
        key::Code::KP_4,
        key::Code::KP_5,
        key::Code::KP_6,
        key::Code::KP_Add,
        key::Code::KP_1,
        key::Code::KP_2,
        key::Code::KP_3,
        key::Code::KP_0,
        key::Code::KP_Period,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::UK_Backslash,
        key::Code::F11,
        // 0x60     96
        key::Code::F12,
        key::Code::Unknown,
        key::Code::LANG3,   // Katakana
        key::Code::LANG4,   // Hiragana
        key::Code::Unknown, // Henkan
        key::Code::Unknown, // Hiragana_Katakana
        key::Code::Unknown, // Muhenkan
        key::Code::Unknown,
        key::Code::KP_Enter,
        key::Code::RightCtrl,
        key::Code::KP_Divide,
        key::Code::PrintScreen,
        key::Code::RightAlt,
        key::Code::Unknown, // line feed
        key::Code::Home,
        key::Code::Up,
        // 0x70     112
        key::Code::PageUp,
        key::Code::Left,
        key::Code::Right,
        key::Code::End,
        key::Code::Down,
        key::Code::PageDown,
        key::Code::Insert,
        key::Code::Delete,
        key::Code::Unknown,
        key::Code::Mute,
        key::Code::VolumeDown,
        key::Code::VolumeUp,
        key::Code::Unknown, // power off
        key::Code::KP_Equal,
        key::Code::KP_PlusMinus,
        key::Code::Pause,
        // 0x80     128
        key::Code::Unknown, // launch A
        key::Code::KP_Decimal,
        key::Code::LANG1, // hangul
        key::Code::LANG2, // hangul/hanja toggle
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Menu,
        key::Code::Cancel,
        key::Code::Again,
        key::Code::Unknown, // SunProps
        key::Code::Undo,
        key::Code::Unknown, // SunFront
        key::Code::Copy,
        key::Code::Unknown, // Open
        key::Code::Paste,
        // 0x90     144
        key::Code::Find,
        key::Code::Cut,
        key::Code::Help,
        key::Code::Unknown, // XF86MenuKB
        key::Code::Unknown, // XF86Calculator
        key::Code::Unknown,
        key::Code::Unknown, //XF86Sleep
        key::Code::Unknown, //XF86Wakeup
        key::Code::Unknown, //XF86Explorer
        key::Code::Unknown, //XF86Send
        key::Code::Unknown,
        key::Code::Unknown, //Xfer
        key::Code::Unknown, //launch1
        key::Code::Unknown, //launch2
        key::Code::Unknown, //WWW
        key::Code::Unknown, //DOS
        // 0xA0     160
        key::Code::Unknown, // Screensaver
        key::Code::Unknown,
        key::Code::Unknown, // RotateWindows
        key::Code::Unknown, // Mail
        key::Code::Unknown, // Favorites
        key::Code::Unknown, // MyComputer
        key::Code::Unknown, // Back
        key::Code::Unknown, // Forward
        key::Code::Unknown,
        key::Code::Unknown, // Eject
        key::Code::Unknown, // Eject
        key::Code::Unknown, // AudioNext
        key::Code::Unknown, // AudioPlay
        key::Code::Unknown, // AudioPrev
        key::Code::Unknown, // AudioStop
        key::Code::Unknown, // AudioRecord
        // 0xB0     176
        key::Code::Unknown, // AudioRewind
        key::Code::Unknown, // Phone
        key::Code::Unknown,
        key::Code::Unknown, // Tools
        key::Code::Unknown, // HomePage
        key::Code::Unknown, // Reload
        key::Code::Unknown, // Close
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown, // ScrollUp
        key::Code::Unknown, // ScrollDown
        key::Code::Unknown, // parentleft
        key::Code::Unknown, // parentright
        key::Code::Unknown, // New
        key::Code::Unknown, // Redo
        key::Code::Unknown, // Tools
        // 0xC0     192
        key::Code::Unknown, // Launch5
        key::Code::Unknown, // Launch6
        key::Code::Unknown, // Launch7
        key::Code::Unknown, // Launch8
        key::Code::Unknown, // Launch9
        key::Code::Unknown,
        key::Code::Unknown, // AudioMicMute
        key::Code::Unknown, // TouchpadToggle
        key::Code::Unknown, // TouchpadPadOn
        key::Code::Unknown, // TouchpadOff
        key::Code::Unknown,
        key::Code::Unknown, // Mode_switch
        key::Code::Unknown, // Alt_L
        key::Code::Unknown, // Meta_L
        key::Code::Unknown, // Super_L
        key::Code::Unknown, // Hyper_L
        // 0xD0     208
        key::Code::Unknown, // AudioPlay
        key::Code::Unknown, // AudioPause
        key::Code::Unknown, // Launch3
        key::Code::Unknown, // Launch4
        key::Code::Unknown, // LaunchB
        key::Code::Unknown, // Suspend
        key::Code::Unknown, // Close
        key::Code::Unknown, // AudioPlay
        key::Code::Unknown, // AudioForward
        key::Code::Unknown,
        key::Code::Unknown, // Print
        key::Code::Unknown,
        key::Code::Unknown, // WebCam
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown, // Mail
        // 0xE0     224
        key::Code::Unknown, // Messenger
        key::Code::Unknown, // Seach
        key::Code::Unknown, // GO
        key::Code::Unknown, // Finance
        key::Code::Unknown, // Game
        key::Code::Unknown, // Shop
        key::Code::Unknown,
        key::Code::Unknown, // Cancel
        key::Code::Unknown, // MonBrightnessDown
        key::Code::Unknown, // MonBrightnessUp
        key::Code::Unknown, // AudioMedia
        key::Code::Unknown, // Display
        key::Code::Unknown, // KbdLightOnOff
        key::Code::Unknown, // KbdBrightnessDown
        key::Code::Unknown, // KbdBrightnessUp
        key::Code::Unknown, // Send
        // 0xF0     240
        key::Code::Unknown, // Reply
        key::Code::Unknown, // MailForward
        key::Code::Unknown, // Save
        key::Code::Unknown, // Documents
        key::Code::Unknown, // Battery
        key::Code::Unknown, // Bluetooth
        key::Code::Unknown, // WLan
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
        key::Code::Unknown,
    ]
}

fn build_keysym_map() -> HashMap<u32, key::Sym> {
    let mut map = HashMap::new();

    map.insert(xkb::KEY_Escape, key::Sym::Escape);
    map.insert(xkb::KEY_Tab, key::Sym::Tab);
    map.insert(xkb::KEY_ISO_Left_Tab, key::Sym::LeftTab);
    map.insert(xkb::KEY_BackSpace, key::Sym::Backspace);
    map.insert(xkb::KEY_Return, key::Sym::Return);
    map.insert(xkb::KEY_Insert, key::Sym::Insert);
    map.insert(xkb::KEY_Delete, key::Sym::Delete);
    map.insert(xkb::KEY_Clear, key::Sym::Delete);
    map.insert(xkb::KEY_Pause, key::Sym::Pause);
    map.insert(xkb::KEY_Print, key::Sym::Print);
    map.insert(0x1005FF60, key::Sym::SysRq); // hardcoded Sun SysReq
    map.insert(0x1007ff00, key::Sym::SysRq); // hardcoded X386 SysReq

    // cursor movement

    map.insert(xkb::KEY_Home, key::Sym::Home);
    map.insert(xkb::KEY_End, key::Sym::End);
    map.insert(xkb::KEY_Left, key::Sym::Left);
    map.insert(xkb::KEY_Up, key::Sym::Up);
    map.insert(xkb::KEY_Right, key::Sym::Right);
    map.insert(xkb::KEY_Down, key::Sym::Down);
    map.insert(xkb::KEY_Page_Up, key::Sym::PageUp);
    map.insert(xkb::KEY_Page_Down, key::Sym::PageDown);
    map.insert(xkb::KEY_Prior, key::Sym::PageUp);
    map.insert(xkb::KEY_Next, key::Sym::PageDown);

    // modifiers

    map.insert(xkb::KEY_Shift_L, key::Sym::LeftShift);
    map.insert(xkb::KEY_Shift_R, key::Sym::RightShift);
    map.insert(xkb::KEY_Shift_Lock, key::Sym::Shift);
    map.insert(xkb::KEY_Control_L, key::Sym::LeftCtrl);
    map.insert(xkb::KEY_Control_R, key::Sym::RightCtrl);
    map.insert(xkb::KEY_Meta_L, key::Sym::LeftMeta);
    map.insert(xkb::KEY_Meta_R, key::Sym::RightMeta);
    map.insert(xkb::KEY_Alt_L, key::Sym::LeftAlt);
    map.insert(xkb::KEY_Alt_R, key::Sym::RightAlt);
    map.insert(xkb::KEY_Caps_Lock, key::Sym::CapsLock);
    map.insert(xkb::KEY_Num_Lock, key::Sym::NumLock);
    map.insert(xkb::KEY_Scroll_Lock, key::Sym::ScrollLock);
    map.insert(xkb::KEY_Super_L, key::Sym::LeftSuper);
    map.insert(xkb::KEY_Super_R, key::Sym::RightSuper);
    map.insert(xkb::KEY_Menu, key::Sym::Menu);
    map.insert(xkb::KEY_Help, key::Sym::Help);
    map.insert(0x1000FF74, key::Sym::LeftTab); // hardcoded HP backtab
    map.insert(0x1005FF10, key::Sym::F11); // hardcoded Sun F36 (labeled F11)
    map.insert(0x1005FF11, key::Sym::F12); // hardcoded Sun F37 (labeled F12)

    // numeric and function keypad keys

    map.insert(xkb::KEY_KP_Enter, key::Sym::KP_Enter);
    map.insert(xkb::KEY_KP_Delete, key::Sym::KP_Delete);
    map.insert(xkb::KEY_KP_Home, key::Sym::KP_Home);
    map.insert(xkb::KEY_KP_Begin, key::Sym::KP_Begin);
    map.insert(xkb::KEY_KP_End, key::Sym::KP_End);
    map.insert(xkb::KEY_KP_Page_Up, key::Sym::KP_PageUp);
    map.insert(xkb::KEY_KP_Page_Down, key::Sym::KP_PageDown);
    map.insert(xkb::KEY_KP_Up, key::Sym::KP_Up);
    map.insert(xkb::KEY_KP_Down, key::Sym::KP_Down);
    map.insert(xkb::KEY_KP_Left, key::Sym::KP_Left);
    map.insert(xkb::KEY_KP_Right, key::Sym::KP_Right);
    map.insert(xkb::KEY_KP_Equal, key::Sym::KP_Equal);
    map.insert(xkb::KEY_KP_Multiply, key::Sym::KP_Multiply);
    map.insert(xkb::KEY_KP_Add, key::Sym::KP_Add);
    map.insert(xkb::KEY_KP_Divide, key::Sym::KP_Divide);
    map.insert(xkb::KEY_KP_Subtract, key::Sym::KP_Subtract);
    map.insert(xkb::KEY_KP_Decimal, key::Sym::KP_Decimal);
    map.insert(xkb::KEY_KP_Separator, key::Sym::KP_Separator);

    map.insert(xkb::KEY_KP_0, key::Sym::KP_0);
    map.insert(xkb::KEY_KP_1, key::Sym::KP_1);
    map.insert(xkb::KEY_KP_2, key::Sym::KP_2);
    map.insert(xkb::KEY_KP_3, key::Sym::KP_3);
    map.insert(xkb::KEY_KP_4, key::Sym::KP_4);
    map.insert(xkb::KEY_KP_6, key::Sym::KP_6);
    map.insert(xkb::KEY_KP_7, key::Sym::KP_7);
    map.insert(xkb::KEY_KP_8, key::Sym::KP_8);
    map.insert(xkb::KEY_KP_9, key::Sym::KP_9);

    // International input method support keys

    // International & multi-key character composition
    map.insert(xkb::KEY_ISO_Level3_Shift, key::Sym::RightAlt); // AltGr
                                                               //map.insert(xkb::KEY_Multi_key,                 key::Sym::Multi_key);
                                                               //map.insert(xkb::KEY_Codeinput,                 key::Sym::Codeinput);
                                                               //map.insert(xkb::KEY_SingleCandidate,           key::Sym::SingleCandidate);
                                                               //map.insert(xkb::KEY_MultipleCandidate,         key::Sym::MultipleCandidate);
                                                               //map.insert(xkb::KEY_PreviousCandidate,         key::Sym::PreviousCandidate);

    // Misc Functions
    map.insert(xkb::KEY_Mode_switch, key::Sym::ModeSwitch);

    //// Japanese keyboard support
    //map.insert(xkb::KEY_Kanji,                     key::Sym::Kanji);
    //map.insert(xkb::KEY_Muhenkan,                  key::Sym::Muhenkan);
    ////map.insert(xkb::KEY_Henkan_Mode,             key::Sym::Henkan_Mode);
    //map.insert(xkb::KEY_Henkan_Mode,               key::Sym::Henkan);
    //map.insert(xkb::KEY_Henkan,                    key::Sym::Henkan);
    //map.insert(xkb::KEY_Romaji,                    key::Sym::Romaji);
    //map.insert(xkb::KEY_Hiragana,                  key::Sym::Hiragana);
    //map.insert(xkb::KEY_Katakana,                  key::Sym::Katakana);
    //map.insert(xkb::KEY_Hiragana_Katakana,         key::Sym::Hiragana_Katakana);
    //map.insert(xkb::KEY_Zenkaku,                   key::Sym::Zenkaku);
    //map.insert(xkb::KEY_Hankaku,                   key::Sym::Hankaku);
    //map.insert(xkb::KEY_Zenkaku_Hankaku,           key::Sym::Zenkaku_Hankaku);
    //map.insert(xkb::KEY_Touroku,                   key::Sym::Touroku);
    //map.insert(xkb::KEY_Massyo,                    key::Sym::Massyo);
    //map.insert(xkb::KEY_Kana_Lock,                 key::Sym::Kana_Lock);
    //map.insert(xkb::KEY_Kana_Shift,                key::Sym::Kana_Shift);
    //map.insert(xkb::KEY_Eisu_Shift,                key::Sym::Eisu_Shift);
    //map.insert(xkb::KEY_Eisu_toggle,               key::Sym::Eisu_toggle);
    ////map.insert(xkb::KEY_Kanji_Bangou,            key::Sym::Kanji_Bangou);
    ////map.insert(xkb::KEY_Zen_Koho,                key::Sym::Zen_Koho);
    ////map.insert(xkb::KEY_Mae_Koho,                key::Sym::Mae_Koho);
    //map.insert(xkb::KEY_Kanji_Bangou,              key::Sym::Codeinput);
    //map.insert(xkb::KEY_Zen_Koho,                  key::Sym::MultipleCandidate);
    //map.insert(xkb::KEY_Mae_Koho,                  key::Sym::PreviousCandidate);

    //// Korean keyboard support
    //map.insert(xkb::KEY_Hangul,                    key::Sym::Hangul);
    //map.insert(xkb::KEY_Hangul_Start,              key::Sym::Hangul_Start);
    //map.insert(xkb::KEY_Hangul_End,                key::Sym::Hangul_End);
    //map.insert(xkb::KEY_Hangul_Hanja,              key::Sym::Hangul_Hanja);
    //map.insert(xkb::KEY_Hangul_Jamo,               key::Sym::Hangul_Jamo);
    //map.insert(xkb::KEY_Hangul_Romaja,             key::Sym::Hangul_Romaja);
    ////map.insert(xkb::KEY_Hangul_Codeinput,        key::Sym::Hangul_Codeinput);
    //map.insert(xkb::KEY_Hangul_Codeinput,          key::Sym::Codeinput);
    //map.insert(xkb::KEY_Hangul_Jeonja,             key::Sym::Hangul_Jeonja);
    //map.insert(xkb::KEY_Hangul_Banja,              key::Sym::Hangul_Banja);
    //map.insert(xkb::KEY_Hangul_PreHanja,           key::Sym::Hangul_PreHanja);
    //map.insert(xkb::KEY_Hangul_PostHanja,          key::Sym::Hangul_PostHanja);
    ////map.insert(xkb::KEY_Hangul_SingleCandidate,  key::Sym::Hangul_SingleCandidate);
    ////map.insert(xkb::KEY_Hangul_MultipleCandidate, ey.Hangul_MultipleCandidate);
    ////map.insert(xkb::KEY_Hangul_PreviousCandidate, ey.Hangul_PreviousCandidate);
    //map.insert(xkb::KEY_Hangul_SingleCandidate,    key::Sym::SingleCandidate);
    //map.insert(xkb::KEY_Hangul_MultipleCandidate,  key::Sym::MultipleCandidate);
    //map.insert(xkb::KEY_Hangul_PreviousCandidate,  key::Sym::PreviousCandidate);
    //map.insert(xkb::KEY_Hangul_Special,            key::Sym::Hangul_Special);
    ////map.insert(xkb::KEY_Hangul_switch,           key::Sym::Hangul_switch);
    //map.insert(xkb::KEY_Hangul_switch,             key::Sym::Mode_switch);

    // Special keys from X.org - This include multimedia keys,
    // wireless/bluetooth/uwb keys, special launcher keys, etc.
    map.insert(xkb::KEY_XF86Back, key::Sym::Back);
    map.insert(xkb::KEY_XF86Forward, key::Sym::Forward);
    map.insert(xkb::KEY_XF86Stop, key::Sym::Stop);
    map.insert(xkb::KEY_XF86Refresh, key::Sym::Refresh);
    map.insert(xkb::KEY_XF86Favorites, key::Sym::Favorites);
    map.insert(xkb::KEY_XF86AudioMedia, key::Sym::LaunchMedia);
    map.insert(xkb::KEY_XF86OpenURL, key::Sym::OpenUrl);
    map.insert(xkb::KEY_XF86HomePage, key::Sym::HomePage);
    map.insert(xkb::KEY_XF86Search, key::Sym::Search);
    map.insert(xkb::KEY_XF86AudioLowerVolume, key::Sym::VolumeDown);
    map.insert(xkb::KEY_XF86AudioMute, key::Sym::VolumeMute);
    map.insert(xkb::KEY_XF86AudioRaiseVolume, key::Sym::VolumeUp);
    map.insert(xkb::KEY_XF86AudioPlay, key::Sym::MediaPlay);
    map.insert(xkb::KEY_XF86AudioStop, key::Sym::MediaStop);
    map.insert(xkb::KEY_XF86AudioPrev, key::Sym::MediaPrevious);
    map.insert(xkb::KEY_XF86AudioNext, key::Sym::MediaNext);
    map.insert(xkb::KEY_XF86AudioRecord, key::Sym::MediaRecord);
    map.insert(xkb::KEY_XF86AudioPause, key::Sym::MediaPause);
    map.insert(xkb::KEY_XF86Mail, key::Sym::LaunchMail);
    map.insert(xkb::KEY_XF86MyComputer, key::Sym::MyComputer);
    map.insert(xkb::KEY_XF86Calculator, key::Sym::Calculator);
    map.insert(xkb::KEY_XF86Memo, key::Sym::Memo);
    map.insert(xkb::KEY_XF86ToDoList, key::Sym::ToDoList);
    map.insert(xkb::KEY_XF86Calendar, key::Sym::Calendar);
    map.insert(xkb::KEY_XF86PowerDown, key::Sym::PowerDown);
    map.insert(xkb::KEY_XF86ContrastAdjust, key::Sym::ContrastAdjust);
    map.insert(xkb::KEY_XF86Standby, key::Sym::Standby);
    map.insert(xkb::KEY_XF86MonBrightnessUp, key::Sym::MonBrightnessUp);
    map.insert(xkb::KEY_XF86MonBrightnessDown, key::Sym::MonBrightnessDown);
    map.insert(xkb::KEY_XF86KbdLightOnOff, key::Sym::KeyboardLightOnOff);
    map.insert(xkb::KEY_XF86KbdBrightnessUp, key::Sym::KeyboardBrightnessUp);
    map.insert(
        xkb::KEY_XF86KbdBrightnessDown,
        key::Sym::KeyboardBrightnessDown,
    );
    map.insert(xkb::KEY_XF86PowerOff, key::Sym::PowerOff);
    map.insert(xkb::KEY_XF86WakeUp, key::Sym::WakeUp);
    map.insert(xkb::KEY_XF86Eject, key::Sym::Eject);
    map.insert(xkb::KEY_XF86ScreenSaver, key::Sym::ScreenSaver);
    map.insert(xkb::KEY_XF86WWW, key::Sym::WWW);
    map.insert(xkb::KEY_XF86Sleep, key::Sym::Sleep);
    map.insert(xkb::KEY_XF86LightBulb, key::Sym::LightBulb);
    map.insert(xkb::KEY_XF86Shop, key::Sym::Shop);
    map.insert(xkb::KEY_XF86History, key::Sym::History);
    map.insert(xkb::KEY_XF86AddFavorite, key::Sym::AddFavorite);
    map.insert(xkb::KEY_XF86HotLinks, key::Sym::HotLinks);
    map.insert(xkb::KEY_XF86BrightnessAdjust, key::Sym::BrightnessAdjust);
    map.insert(xkb::KEY_XF86Finance, key::Sym::Finance);
    map.insert(xkb::KEY_XF86Community, key::Sym::Community);
    map.insert(xkb::KEY_XF86AudioRewind, key::Sym::AudioRewind);
    map.insert(xkb::KEY_XF86BackForward, key::Sym::BackForward);
    map.insert(xkb::KEY_XF86ApplicationLeft, key::Sym::ApplicationLeft);
    map.insert(xkb::KEY_XF86ApplicationRight, key::Sym::ApplicationRight);
    map.insert(xkb::KEY_XF86Book, key::Sym::Book);
    map.insert(xkb::KEY_XF86CD, key::Sym::CD);
    map.insert(xkb::KEY_XF86Calculater, key::Sym::Calculator);
    map.insert(xkb::KEY_XF86Clear, key::Sym::Clear);
    map.insert(xkb::KEY_XF86ClearGrab, key::Sym::ClearGrab);
    map.insert(xkb::KEY_XF86Close, key::Sym::Close);
    map.insert(xkb::KEY_XF86Copy, key::Sym::Copy);
    map.insert(xkb::KEY_XF86Cut, key::Sym::Cut);
    map.insert(xkb::KEY_XF86Display, key::Sym::Display);
    map.insert(xkb::KEY_XF86DOS, key::Sym::DOS);
    map.insert(xkb::KEY_XF86Documents, key::Sym::Documents);
    map.insert(xkb::KEY_XF86Excel, key::Sym::Excel);
    map.insert(xkb::KEY_XF86Explorer, key::Sym::Explorer);
    map.insert(xkb::KEY_XF86Game, key::Sym::Game);
    map.insert(xkb::KEY_XF86Go, key::Sym::Go);
    map.insert(xkb::KEY_XF86iTouch, key::Sym::iTouch);
    map.insert(xkb::KEY_XF86LogOff, key::Sym::LogOff);
    map.insert(xkb::KEY_XF86Market, key::Sym::Market);
    map.insert(xkb::KEY_XF86Meeting, key::Sym::Meeting);
    map.insert(xkb::KEY_XF86MenuKB, key::Sym::MenuKB);
    map.insert(xkb::KEY_XF86MenuPB, key::Sym::MenuPB);
    map.insert(xkb::KEY_XF86MySites, key::Sym::MySites);
    map.insert(xkb::KEY_XF86New, key::Sym::New);
    map.insert(xkb::KEY_XF86News, key::Sym::News);
    map.insert(xkb::KEY_XF86OfficeHome, key::Sym::OfficeHome);
    map.insert(xkb::KEY_XF86Open, key::Sym::Open);
    map.insert(xkb::KEY_XF86Option, key::Sym::Option);
    map.insert(xkb::KEY_XF86Paste, key::Sym::Paste);
    map.insert(xkb::KEY_XF86Phone, key::Sym::Phone);
    map.insert(xkb::KEY_XF86Reply, key::Sym::Reply);
    map.insert(xkb::KEY_XF86Reload, key::Sym::Reload);
    map.insert(xkb::KEY_XF86RotateWindows, key::Sym::RotateWindows);
    map.insert(xkb::KEY_XF86RotationPB, key::Sym::RotationPB);
    map.insert(xkb::KEY_XF86RotationKB, key::Sym::RotationKB);
    map.insert(xkb::KEY_XF86Save, key::Sym::Save);
    map.insert(xkb::KEY_XF86Send, key::Sym::Send);
    map.insert(xkb::KEY_XF86Spell, key::Sym::Spell);
    map.insert(xkb::KEY_XF86SplitScreen, key::Sym::SplitScreen);
    map.insert(xkb::KEY_XF86Support, key::Sym::Support);
    map.insert(xkb::KEY_XF86TaskPane, key::Sym::TaskPane);
    map.insert(xkb::KEY_XF86Terminal, key::Sym::Terminal);
    map.insert(xkb::KEY_XF86Tools, key::Sym::Tools);
    map.insert(xkb::KEY_XF86Travel, key::Sym::Travel);
    map.insert(xkb::KEY_XF86Video, key::Sym::Video);
    map.insert(xkb::KEY_XF86Word, key::Sym::Word);
    map.insert(xkb::KEY_XF86Xfer, key::Sym::Xfer);
    map.insert(xkb::KEY_XF86ZoomIn, key::Sym::ZoomIn);
    map.insert(xkb::KEY_XF86ZoomOut, key::Sym::ZoomOut);
    map.insert(xkb::KEY_XF86Away, key::Sym::Away);
    map.insert(xkb::KEY_XF86Messenger, key::Sym::Messenger);
    map.insert(xkb::KEY_XF86WebCam, key::Sym::WebCam);
    map.insert(xkb::KEY_XF86MailForward, key::Sym::MailForward);
    map.insert(xkb::KEY_XF86Pictures, key::Sym::Pictures);
    map.insert(xkb::KEY_XF86Music, key::Sym::Music);
    map.insert(xkb::KEY_XF86Battery, key::Sym::Battery);
    map.insert(xkb::KEY_XF86Bluetooth, key::Sym::Bluetooth);
    map.insert(xkb::KEY_XF86WLAN, key::Sym::WLAN);
    map.insert(xkb::KEY_XF86UWB, key::Sym::UWB);
    map.insert(xkb::KEY_XF86AudioForward, key::Sym::AudioForward);
    map.insert(xkb::KEY_XF86AudioRepeat, key::Sym::AudioRepeat);
    map.insert(xkb::KEY_XF86AudioRandomPlay, key::Sym::AudioRandomPlay);
    map.insert(xkb::KEY_XF86Subtitle, key::Sym::Subtitle);
    map.insert(xkb::KEY_XF86AudioCycleTrack, key::Sym::AudioCycleTrack);
    map.insert(xkb::KEY_XF86Time, key::Sym::Time);
    map.insert(xkb::KEY_XF86Select, key::Sym::Select);
    map.insert(xkb::KEY_XF86View, key::Sym::View);
    map.insert(xkb::KEY_XF86TopMenu, key::Sym::TopMenu);
    map.insert(xkb::KEY_XF86Red, key::Sym::Red);
    map.insert(xkb::KEY_XF86Green, key::Sym::Green);
    map.insert(xkb::KEY_XF86Yellow, key::Sym::Yellow);
    map.insert(xkb::KEY_XF86Blue, key::Sym::Blue);
    map.insert(xkb::KEY_XF86Bluetooth, key::Sym::Bluetooth);
    map.insert(xkb::KEY_XF86Suspend, key::Sym::Suspend);
    map.insert(xkb::KEY_XF86Hibernate, key::Sym::Hibernate);
    map.insert(xkb::KEY_XF86TouchpadToggle, key::Sym::TouchpadToggle);
    map.insert(xkb::KEY_XF86TouchpadOn, key::Sym::TouchpadOn);
    map.insert(xkb::KEY_XF86TouchpadOff, key::Sym::TouchpadOff);
    map.insert(xkb::KEY_XF86AudioMicMute, key::Sym::MicMute);
    map.insert(xkb::KEY_XF86Launch0, key::Sym::Launch0); // ### Qt 6: remap properly
    map.insert(xkb::KEY_XF86Launch1, key::Sym::Launch1);
    map.insert(xkb::KEY_XF86Launch2, key::Sym::Launch2);
    map.insert(xkb::KEY_XF86Launch3, key::Sym::Launch3);
    map.insert(xkb::KEY_XF86Launch4, key::Sym::Launch4);
    map.insert(xkb::KEY_XF86Launch5, key::Sym::Launch5);
    map.insert(xkb::KEY_XF86Launch6, key::Sym::Launch6);
    map.insert(xkb::KEY_XF86Launch7, key::Sym::Launch7);
    map.insert(xkb::KEY_XF86Launch8, key::Sym::Launch8);
    map.insert(xkb::KEY_XF86Launch9, key::Sym::Launch9);
    map.insert(xkb::KEY_XF86LaunchA, key::Sym::LaunchA);
    map.insert(xkb::KEY_XF86LaunchB, key::Sym::LaunchB);
    map.insert(xkb::KEY_XF86LaunchC, key::Sym::LaunchC);
    map.insert(xkb::KEY_XF86LaunchD, key::Sym::LaunchD);
    map.insert(xkb::KEY_XF86LaunchE, key::Sym::LaunchE);
    map.insert(xkb::KEY_XF86LaunchF, key::Sym::LaunchF);

    map.shrink_to_fit();

    map
}
