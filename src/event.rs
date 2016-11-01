// This file is part of toy_xcb and is released under the terms
// of the MIT license. See included LICENSE.txt file.

use geometry::{ISize, IPoint};
use mouse;
use key;
use window;


#[derive(Debug)]
pub enum Event {
    Show,
    Hide,
    Expose,
    Close,

    Resize          (ISize),
    Move            (IPoint),
    StateChange     (window::State),
    Enter           (IPoint),
    Leave           (IPoint),

    MousePress      ( IPoint, mouse::Buttons, key::Mods ),
    MouseRelease    ( IPoint, mouse::Buttons, key::Mods ),
    MouseMove       ( IPoint, mouse::Buttons, key::Mods ),

    KeyPress        ( key::Sym, key::Code, String ),
    KeyRelease      ( key::Sym, key::Code, String ),
}
