// This file is part of toy_xcb and is released under the terms
// of the MIT license. See included LICENSE.txt file.

use keyboard::Keyboard;
use geometry::{ISize, IPoint};
use event::Event;
use mouse;
use key;

use xcb;

use std::collections::HashMap;


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum State {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

pub struct Window
{
    conn: xcb::Connection,
    atoms: HashMap<Atom, xcb::Atom>,
    def_screen: i32,
    kbd_ev: u8,
    kbd: Keyboard,

    win: xcb::Window,
    title: String,
}

impl Window {

   pub fn new(width: u16, height: u16, title: String) -> Window {

        let (conn, def_screen) = xcb::Connection::connect_with_xlib_display()
                .expect("could not connect to X server");
        conn.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

        let atoms = {
            let mut cookies = Vec::with_capacity(Atom::num_variants());
            for atom in Atom::variants() {
                let atom_name = format!("{:?}", atom);
                cookies.push(
                    xcb::intern_atom(&conn, true, &atom_name)
                );
            }
            let mut atoms = HashMap::with_capacity(Atom::num_variants());
            for (i, atom) in Atom::variants().enumerate() {
                atoms.insert(*atom,
                    match cookies[i].get_reply() {
                        Ok(r) => { r.atom() },
                        Err(_) => {
                            panic!("could not find atom {:?}", atom);
                        }
                    }
                );
            }
            atoms
        };

        let (kbd, kbd_ev, _) = Keyboard::new(&conn);
        let win = {
            let win = conn.generate_id();
            let setup = conn.get_setup();
            let screen = setup.roots().nth(def_screen as usize).unwrap();

            let values = [
                (xcb::CW_BACK_PIXEL,    screen.white_pixel()),

                (xcb::CW_EVENT_MASK,    xcb::EVENT_MASK_KEY_PRESS |
                                        xcb::EVENT_MASK_KEY_RELEASE |
                                        xcb::EVENT_MASK_BUTTON_PRESS |
                                        xcb::EVENT_MASK_BUTTON_RELEASE |
                                        xcb::EVENT_MASK_ENTER_WINDOW |
                                        xcb::EVENT_MASK_LEAVE_WINDOW |
                                        xcb::EVENT_MASK_POINTER_MOTION |
                                        xcb::EVENT_MASK_BUTTON_MOTION |
                                        xcb::EVENT_MASK_EXPOSURE |
                                        xcb::EVENT_MASK_STRUCTURE_NOTIFY |
                                        xcb::EVENT_MASK_PROPERTY_CHANGE),
            ];

            xcb::create_window(&conn, xcb::COPY_FROM_PARENT as u8,
                win, screen.root(), 0, 0, width, height, 0,
                xcb::WINDOW_CLASS_INPUT_OUTPUT as u16, screen.root_visual(),
                &values);

            win
        };


        let wm_delete_window = *atoms.get(&Atom::WM_DELETE_WINDOW).unwrap();
        let wm_protocols = *atoms.get(&Atom::WM_PROTOCOLS).unwrap();

        let values = [wm_delete_window];
        xcb::change_property(&conn, xcb::PROP_MODE_REPLACE as u8,
                win, wm_protocols, xcb::ATOM_ATOM, 32, &values);

        // setting title
        if !title.is_empty() {
            xcb::change_property(&conn, xcb::PROP_MODE_REPLACE as u8, win,
                    xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, title.as_bytes());
        }

        xcb::map_window(&conn, win);
        conn.flush();

        Window {
            conn: conn,
            atoms: atoms,
            def_screen: def_screen,
            kbd_ev: kbd_ev,
            kbd: kbd,
            win: win,
            title: title,
        }
    }

    pub fn wait_event(&self) -> Option<Event> {
        self.conn.wait_for_event().and_then(|ev| self.translate_event(ev))
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }
    pub fn set_title(&mut self, title: String) {
        if title != self.title {
            self.title = title;
            xcb::change_property(&self.conn, xcb::PROP_MODE_REPLACE as u8, self.win,
                    xcb::ATOM_WM_NAME, xcb::ATOM_STRING, 8, self.title.as_bytes());
        }
    }


    fn translate_event(&self, xcb_ev: xcb::GenericEvent) -> Option<Event> {
        let r = xcb_ev.response_type() & !0x80;
        match r {
            xcb::KEY_PRESS => Some( self.kbd.make_key_event(xcb::cast_event(&xcb_ev), true) ),
            xcb::KEY_RELEASE => Some( self.kbd.make_key_event(xcb::cast_event(&xcb_ev), false) ),

            xcb::BUTTON_PRESS => {
                let ev = self.make_mouse_event(xcb::cast_event(&xcb_ev));
                Some(Event::MousePress(ev.0, ev.1, ev.2))
            },
            xcb::BUTTON_RELEASE => {
                let ev = self.make_mouse_event(xcb::cast_event(&xcb_ev));
                Some(Event::MouseRelease(ev.0, ev.1, ev.2))
            },

            xcb::ENTER_NOTIFY => Some(Event::Enter(
                Window::make_enterleave_point(
                    xcb::cast_event(&xcb_ev)
                ))
            ),
            xcb::LEAVE_NOTIFY => Some(Event::Leave(
                Window::make_enterleave_point(
                    xcb::cast_event(&xcb_ev)
                ))
            ),

            xcb::MOTION_NOTIFY => {
                let ev = self.make_mouse_event(xcb::cast_event(&xcb_ev));
                Some(Event::MouseMove(ev.0, ev.1, ev.2))
            },
            xcb::CLIENT_MESSAGE => {
                let wm_protocols = *self.atoms.get(&Atom::WM_PROTOCOLS).unwrap();
                let wm_delete_window = *self.atoms.get(&Atom::WM_DELETE_WINDOW).unwrap();
                let cm_ev: &xcb::ClientMessageEvent = xcb::cast_event(&xcb_ev);
                if cm_ev.type_() == wm_protocols && cm_ev.format() == 32 {
                    let protocol = cm_ev.data().data32()[0];
                    if protocol == wm_delete_window {
                        return Some(Event::Close);
                    }
                }
                None
            }
            _ => {
                if r == self.kbd_ev {
                    let xkb_ev: &XkbGenericEvent = xcb::cast_event(&xcb_ev);
                    if xkb_ev.device_id() == self.kbd.get_device_id() as u8 {
                        match xkb_ev.xkb_type() {
                            xcb::xkb::STATE_NOTIFY => {
                                self.kbd.update_state(xcb::cast_event(&xcb_ev));
                            }
                            _ => {}
                        }
                    }

                    // xkb events do not translate into application event
                    // so once this done, we wait and return the next event
                    self.wait_event()
                }
                else { None }
            }
        }
    }


    fn make_mouse_event(&self, xcb_ev: &xcb::ButtonPressEvent)
            -> ( IPoint, mouse::Buttons, key::Mods )
    {
        let pos = IPoint {
            x: xcb_ev.event_x() as i32,
            y: xcb_ev.event_y() as i32
        };
        let but = match xcb_ev.detail() {
            1 => mouse::Buttons::left(),
            2 => mouse::Buttons::middle(),
            3 => mouse::Buttons::right(),
            _ => mouse::Buttons::none(),
        };

        (pos, but, self.kbd.get_mods())
    }

    fn make_enterleave_point(xcb_ev: &xcb::EnterNotifyEvent) -> IPoint {
        IPoint::new(xcb_ev.event_x() as i32, xcb_ev.event_y() as i32)
    }
}


/// struct that has fields common to the 3 different xkb events
/// (StateNotify, NewKeyboardNotify, MapNotify)
#[repr(C)]
struct xcb_xkb_generic_event_t {
    response_type: u8,
    xkb_type: u8,
    sequence: u16,
    time: xcb::Timestamp,
    device_id: u8,
}

struct XkbGenericEvent {
    base: xcb::Event<xcb_xkb_generic_event_t>
}

impl XkbGenericEvent {
    pub fn response_type(&self) -> u8 {
        unsafe { (*self.base.ptr).response_type }
    }
    #[allow(non_snake_case)]
    pub fn xkb_type(&self) -> u8 {
        unsafe { (*self.base.ptr).xkb_type }
    }
    pub fn sequence(&self) -> u16 {
        unsafe { (*self.base.ptr).sequence }
    }
    pub fn time(&self) -> xcb::Timestamp {
        unsafe { (*self.base.ptr).time }
    }
    #[allow(non_snake_case)]
    pub fn device_id(&self) -> u8 {
        unsafe { (*self.base.ptr).device_id }
    }
}


iterable_key_enum! {
    Atom =>
        UTF8_STRING,

        WM_PROTOCOLS,
        WM_DELETE_WINDOW,
        WM_TRANSIENT_FOR,
        WM_CHANGE_STATE,
        WM_STATE,
        _NET_WM_STATE,
        _NET_WM_STATE_MODAL,
        _NET_WM_STATE_STICKY,
        _NET_WM_STATE_MAXIMIZED_VERT,
        _NET_WM_STATE_MAXIMIZED_HORZ,
        _NET_WM_STATE_SHADED,
        _NET_WM_STATE_SKIP_TASKBAR,
        _NET_WM_STATE_SKIP_PAGER,
        _NET_WM_STATE_HIDDEN,
        _NET_WM_STATE_FULLSCREEN,
        _NET_WM_STATE_ABOVE,
        _NET_WM_STATE_BELOW,
        _NET_WM_STATE_DEMANDS_ATTENTION,
        _NET_WM_STATE_FOCUSED,
        _NET_WM_NAME
}


