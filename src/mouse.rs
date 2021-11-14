// This file is part of toy_xcb and is released under the terms
// of the MIT license. See included LICENSE.txt file.

use std::ops::{BitAnd, BitOr, BitXor};

pub const NONE: u8 = 0;
pub const LEFT: u8 = 1;
pub const MIDDLE: u8 = 2;
pub const RIGHT: u8 = 4;
pub const MASK: u8 = 7;

#[derive(Copy, Clone, PartialEq, Eq, Default, Debug)]
pub struct Buttons {
    fields: u8,
}

impl Buttons {
    pub fn new(fields: u8) -> Buttons {
        Buttons {
            fields: fields & MASK,
        }
    }

    pub fn none() -> Buttons {
        Buttons { fields: NONE }
    }
    pub fn left() -> Buttons {
        Buttons { fields: LEFT }
    }
    pub fn middle() -> Buttons {
        Buttons { fields: MIDDLE }
    }
    pub fn right() -> Buttons {
        Buttons { fields: RIGHT }
    }

    pub fn fields(&self) -> u8 {
        self.fields
    }
    pub fn is_none(&self) -> bool {
        self.fields == NONE
    }

    pub fn is_left(&self) -> bool {
        self.fields == LEFT
    }
    pub fn has_left(&self) -> bool {
        (self.fields & LEFT) != 0
    }

    pub fn is_middle(&self) -> bool {
        self.fields == MIDDLE
    }
    pub fn has_middle(&self) -> bool {
        (self.fields & MIDDLE) != 0
    }

    pub fn is_right(&self) -> bool {
        self.fields == RIGHT
    }
    pub fn has_right(&self) -> bool {
        (self.fields & RIGHT) != 0
    }

    pub fn has_all(&self, fields: u8) -> bool {
        (self.fields & fields) == fields
    }
    pub fn has_any(&self, fields: u8) -> bool {
        (self.fields & fields) != 0
    }
    pub fn has_none(&self, fields: u8) -> bool {
        (self.fields & fields) == 0
    }
}

impl PartialEq<u8> for Buttons {
    fn eq(&self, rhs: &u8) -> bool {
        self.fields == *rhs
    }
}

impl PartialEq<Buttons> for u8 {
    fn eq(&self, rhs: &Buttons) -> bool {
        *self == rhs.fields
    }
}

impl BitAnd for Buttons {
    type Output = Buttons;
    fn bitand(self, rhs: Buttons) -> Buttons {
        Buttons {
            fields: self.fields & rhs.fields,
        }
    }
}

impl BitOr for Buttons {
    type Output = Buttons;
    fn bitor(self, rhs: Buttons) -> Buttons {
        Buttons {
            fields: self.fields | rhs.fields,
        }
    }
}

impl BitXor for Buttons {
    type Output = Buttons;
    fn bitxor(self, rhs: Buttons) -> Buttons {
        Buttons {
            fields: self.fields ^ rhs.fields,
        }
    }
}

#[test]
fn buts_has() {
    let m = Buttons::left() | Buttons::right();

    assert!(m.has_left());
    assert!(m.has_right());
    assert!(!m.has_middle());

    assert!(m.has_all(LEFT | RIGHT));
    assert!(m.has_all(LEFT));
    assert!(m.has_all(RIGHT));
    assert!(!m.has_all(LEFT | RIGHT | MIDDLE));

    assert!(m.has_any(LEFT | RIGHT));
    assert!(m.has_any(LEFT));
    assert!(m.has_any(RIGHT));
    assert!(!m.has_any(MIDDLE));
    assert!(m.has_any(LEFT | RIGHT | MIDDLE));

    assert!(!m.is_none());

    assert!(Buttons::none().is_none());
}

#[test]
fn buts_ops() {
    let m = Buttons::left() & Buttons::middle();

    assert_eq!(Buttons::none(), m);
    assert_eq!(NONE, m.fields());

    let m = Buttons::left() | Buttons::right();
    assert_eq!(Buttons::new(LEFT | RIGHT), m);
}
