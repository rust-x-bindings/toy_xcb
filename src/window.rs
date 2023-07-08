// This file is part of toy_xcb and is released under the terms
// of the MIT license. See included LICENSE.txt file.

use super::event::Event;
use super::geometry::IPoint;
use super::key;
use super::keyboard::Keyboard;
use super::mouse;
use super::Result;

use xcb::x;
use xcb::xkb;
use xcb::{self, Xid};

xcb::atoms_struct! {
    #[derive(Copy, Clone, Debug)]
    pub(crate) struct Atoms {
        pub utf8_string                     => b"UTF8_STRING",
        pub wm_protocols                    => b"WM_PROTOCOLS",
        pub wm_delete_window                => b"WM_DELETE_WINDOW",
        pub wm_transient_for                => b"WM_TRANSIENT_FOR",
        pub wm_change_state                 => b"WM_CHANGE_STATE",
        pub wm_state                        => b"WM_STATE",
        pub net_wm_state                    => b"_NET_WM_STATE",
        pub net_wm_state_modal              => b"_NET_WM_STATE_MODAL",
        pub net_wm_state_sticky             => b"_NET_WM_STATE_STICKY",
        pub net_wm_state_maximized_vert     => b"_NET_WM_STATE_MAXIMIZED_VERT",
        pub net_wm_state_maximized_horz     => b"_NET_WM_STATE_MAXIMIZED_HORZ",
        pub net_wm_state_shaded             => b"_NET_WM_STATE_SHADED",
        pub net_wm_state_skip_taskbar       => b"_NET_WM_STATE_SKIP_TASKBAR",
        pub net_wm_state_skip_pager         => b"_NET_WM_STATE_SKIP_PAGER",
        pub net_wm_state_hidden             => b"_NET_WM_STATE_HIDDEN",
        pub net_wm_state_fullscreen         => b"_NET_WM_STATE_FULLSCREEN",
        pub net_wm_state_above              => b"_NET_WM_STATE_ABOVE",
        pub net_wm_state_below              => b"_NET_WM_STATE_BELOW",
        pub net_wm_state_demands_attention  => b"_NET_WM_STATE_DEMANDS_ATTENTION",
        pub net_wm_state_focused            => b"_NET_WM_STATE_FOCUSED",
        pub net_wm_name                     => b"_NET_WM_NAME",
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum State {
    Normal,
    Minimized,
    Maximized,
    Fullscreen,
    Hidden,
}

pub struct Window {
    conn: xcb::Connection,
    atoms: Atoms,
    def_screen: i32,
    kbd: Keyboard,

    win: x::Window,
    title: String,
}

impl Window {
    pub fn new(width: u16, height: u16, title: String) -> Result<Window> {
        let (conn, def_screen) =
            xcb::Connection::connect_with_xlib_display_and_extensions(&[xcb::Extension::Xkb], &[])?;
        conn.set_event_queue_owner(xcb::EventQueueOwner::Xcb);

        let atoms = Atoms::intern_all(&conn)?;

        let kbd = Keyboard::new(&conn)?;
        let win = {
            let win = conn.generate_id();
            let setup = conn.get_setup();
            let screen = setup.roots().nth(def_screen as usize).unwrap();

            conn.check_request(conn.send_request_checked(&x::CreateWindow {
                depth: x::COPY_FROM_PARENT as u8,
                wid: win,
                parent: screen.root(),
                x: 0,
                y: 0,
                width,
                height,
                border_width: 0,
                class: x::WindowClass::InputOutput,
                visual: screen.root_visual(),
                value_list: &[
                    x::Cw::BackPixel(screen.white_pixel()),
                    x::Cw::EventMask(
                        x::EventMask::KEY_PRESS
                            | x::EventMask::KEY_RELEASE
                            | x::EventMask::BUTTON_PRESS
                            | x::EventMask::BUTTON_RELEASE
                            | x::EventMask::ENTER_WINDOW
                            | x::EventMask::LEAVE_WINDOW
                            | x::EventMask::POINTER_MOTION
                            | x::EventMask::BUTTON_MOTION
                            | x::EventMask::EXPOSURE
                            | x::EventMask::STRUCTURE_NOTIFY
                            | x::EventMask::PROPERTY_CHANGE,
                    ),
                ],
            }))?;

            win
        };

        conn.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: win,
            property: atoms.wm_protocols,
            r#type: x::ATOM_ATOM,
            data: &[atoms.wm_delete_window],
        });

        // setting title
        if !title.is_empty() {
            conn.send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: win,
                property: x::ATOM_WM_NAME,
                r#type: x::ATOM_STRING,
                data: title.as_bytes(),
            });
        }

        conn.send_request(&x::MapWindow { window: win });
        conn.flush()?;

        Ok(Window {
            conn: conn,
            atoms: atoms,
            def_screen: def_screen,
            kbd,
            win: win,
            title: title,
        })
    }

    pub fn wait_event(&self) -> Result<Event> {
        let xcb_ev = self.conn.wait_for_event()?;
        match self.translate_event(xcb_ev) {
            Some(ev) => Ok(ev),
            None => self.wait_event(),
        }
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn set_title(&mut self, title: String) {
        if title != self.title {
            self.title = title;
            self.conn.send_request(&x::ChangeProperty {
                mode: x::PropMode::Replace,
                window: self.win,
                property: x::ATOM_WM_NAME,
                r#type: x::ATOM_STRING,
                data: self.title.as_bytes(),
            });
            self.conn.flush().unwrap(); // should probably return a result
        }
    }

    pub fn default_screen(&self) -> usize {
        self.def_screen as usize
    }

    fn translate_event(&self, xcb_ev: xcb::Event) -> Option<Event> {
        match xcb_ev {
            xcb::Event::X(x::Event::KeyPress(xcb_ev)) => {
                Some(self.kbd.make_key_event(&xcb_ev, true))
            }
            xcb::Event::X(x::Event::KeyRelease(xcb_ev)) => {
                Some(self.kbd.make_key_event(&xcb_ev, false))
            }
            xcb::Event::X(x::Event::ButtonPress(xcb_ev)) => {
                let ev = self.make_mouse_event(&xcb_ev);
                Some(Event::MousePress(ev.0, ev.1, ev.2))
            }
            xcb::Event::X(x::Event::ButtonRelease(xcb_ev)) => {
                let ev = self.make_mouse_event(&xcb_ev);
                Some(Event::MouseRelease(ev.0, ev.1, ev.2))
            }
            xcb::Event::X(x::Event::EnterNotify(xcb_ev)) => {
                Some(Event::Enter(Window::make_enterleave_point(&xcb_ev)))
            }
            xcb::Event::X(x::Event::LeaveNotify(xcb_ev)) => {
                Some(Event::Leave(Window::make_enterleave_point(&xcb_ev)))
            }
            xcb::Event::X(x::Event::MotionNotify(xcb_ev)) => {
                let point = IPoint {
                    x: xcb_ev.event_x() as _,
                    y: xcb_ev.event_y() as _,
                };
                let buttons = translate_buttons(xcb_ev.state());
                let mods = self.kbd.get_mods();
                Some(Event::MouseMove(point, buttons, mods))
            }
            xcb::Event::X(x::Event::ClientMessage(xcb_ev)) => {
                if xcb_ev.r#type() == self.atoms.wm_protocols {
                    if let x::ClientMessageData::Data32([protocol, ..]) = xcb_ev.data() {
                        if protocol == self.atoms.wm_delete_window.resource_id() {
                            return Some(Event::Close);
                        }
                    }
                }
                None
            }
            xcb::Event::Xkb(xkb::Event::StateNotify(xcb_ev)) => {
                if xcb_ev.device_id() as i32 == self.kbd.get_device_id() {
                    self.kbd.update_state(&xcb_ev);
                }
                None
            }
            _ => None,
        }
    }

    fn make_mouse_event(
        &self,
        xcb_ev: &x::ButtonPressEvent,
    ) -> (IPoint, mouse::Buttons, key::Mods) {
        let pos = IPoint {
            x: xcb_ev.event_x() as i32,
            y: xcb_ev.event_y() as i32,
        };

        (pos, translate_buttons(xcb_ev.state()), self.kbd.get_mods())
    }

    fn make_enterleave_point(xcb_ev: &x::EnterNotifyEvent) -> IPoint {
        IPoint::new(xcb_ev.event_x() as i32, xcb_ev.event_y() as i32)
    }
}

fn translate_buttons(xcb_state: x::KeyButMask) -> mouse::Buttons {
    let mut but = mouse::Buttons::empty();
    if xcb_state.contains(x::KeyButMask::BUTTON1) {
        but |= mouse::Buttons::LEFT;
    }
    if xcb_state.contains(x::KeyButMask::BUTTON2) {
        but |= mouse::Buttons::MIDDLE;
    }
    if xcb_state.contains(x::KeyButMask::BUTTON3) {
        but |= mouse::Buttons::RIGHT;
    }
    but
}
