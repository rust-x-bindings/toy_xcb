// This file is part of toy_xcb and is released under the terms
// of the MIT license. See included LICENSE.txt file.

use bitflags::bitflags;

bitflags! {
    pub struct Buttons: u8 {
       const LEFT = 1;
       const MIDDLE = 2;
       const RIGHT = 4;
    }
}
