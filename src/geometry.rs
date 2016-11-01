
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Eq;


pub type FPoint = Point<f32>;
pub type IPoint = Point<i32>;

pub type FSize = Size<f32>;
pub type ISize = Size<i32>;

pub type FRect = Rect<f32>;
pub type IRect = Rect<i32>;

pub type FMargins = Margins<f32>;
pub type IMargins = Margins<i32>;



#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Point<T: Copy> {
    pub x: T,
    pub y: T,
}

impl<T: Copy> Point<T> {
    pub fn new(x: T, y: T) -> Point<T> {
        Point { x: x, y: y }
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Size<T: Copy> {
    pub w: T,
    pub h: T,
}

impl<T: Copy> Size<T> {
    pub fn new(w: T, h: T) -> Size<T> {
        Size { w: w, h: h }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rect<T: Copy> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T: Copy> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Rect<T> {
        Rect {
            x: x, y: y, w: w, h: h,
        }
    }
    pub fn new_s(x: T, y: T, size: Size<T>) -> Rect<T> {
        Rect {
            x: x, y: y,
            w: size.w, h: size.h,
        }
    }
    pub fn new_p(point: Point<T>, w: T, h: T) -> Rect<T> {
        Rect {
            x: point.x, y: point.y,
            w: w, h: h,
        }
    }
    pub fn new_ps(point: Point<T>, size: Size<T>) -> Rect<T> {
        Rect {
            x: point.x, y: point.y,
            w: size.w, h: size.h,
        }
    }

    pub fn point(&self) -> Point<T> {
        Point {
            x: self.x, y: self.y,
        }
    }
    pub fn size(&self) -> Size<T> {
        Size {
            w: self.w, h: self.h,
        }
    }
}


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Margins<T : Copy> {
    pub l: T,
    pub r: T,
    pub t: T,
    pub b: T,
}

impl<T: Copy> Margins<T> {
    fn new (l: T, r: T, t: T, b: T) -> Margins<T> {
        Margins {
            l: l, r: r, t: t, b: b,
        }
    }
}


pub trait HasArea {
    type Output;

    fn area(&self) -> Self::Output;
}

impl<T> HasArea for Size<T>
        where T : Mul<Output=T> + Copy {
    type Output = T;

    fn area(&self) -> T {
        self.w * self.h
    }
}

impl<T> HasArea for Rect<T>
        where T : Mul<Output=T> + Copy {
    type Output = T;

    fn area(&self) -> T {
        self.w * self.h
    }
}

impl<T> Add for Point<T>
        where T: Add<Output=T> + Copy {
    type Output = Point<T>;

    fn add(self, rhs: Point<T>) -> Point<T> {
        Point {x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

impl<T> Sub for Point<T>
        where T: Sub<Output=T> + Copy {
    type Output = Point<T>;

    fn sub(self, rhs: Point<T>) -> Point<T> {
        Point {x: self.x - rhs.x, y: self.y - rhs.y}
    }
}

impl<T> Mul<T> for Point<T>
        where T: Mul<Output=T> + Copy {
    type Output = Point<T>;

    fn mul (self, rhs: T) -> Point<T> {
        Point { x: self.x * rhs, y: self.y * rhs }
    }
}
// this doesn't compile with overflow error on `self*rhs.x`.
// not sure why
// impl<T> Mul<Point<T>> for i32
//         where i32: Mul<T, Output=T>, T: Copy {
//     type Output = Point<T>;
//
//     fn mul (self, rhs: Point<T>) -> Point<T> {
//         Point { x: self * rhs.x, y: self * rhs.y }
//     }
// }

impl<T> Div<T> for Point<T>
        where T: Div<Output=T> + Copy {
    type Output = Point<T>;

    fn div(self, rhs: T) -> Point<T> {
        Point { x: self.x / rhs, y: self.y / rhs }
    }
}


impl<T> Add<Margins<T>> for Rect<T>
where T: Add<Output=T> + Sub<Output=T> + Copy {
    type Output = Rect<T>;

    fn add(self, rhs: Margins<T>) -> Rect<T> {
        Rect {
            x: self.x - rhs.l,
            y: self.y - rhs.t,
            w: self.w + (rhs.l + rhs.r),
            h: self.h + (rhs.t + rhs.b),
        }
    }
}


impl<T> Sub<Margins<T>> for Rect<T>
where T: Add<Output=T> + Sub<Output=T> + Copy {
    type Output = Rect<T>;

    fn sub(self, rhs: Margins<T>) -> Rect<T> {
        Rect {
            x: self.x + rhs.l,
            y: self.y + rhs.t,
            w: self.w - (rhs.l + rhs.r),
            h: self.h - (rhs.t + rhs.b),
        }
    }
}



#[test]
fn area() {
    let s = Size { w: 5, h: 4 };
    assert_eq!(20, s.area());

    let r = Rect::new_s(0, 0, s);
    assert_eq!(20, r.area());
}

#[test]
fn ops() {
    let v1 = Point::new(3, 4);
    let v2 = Point::new(6, 2);

    assert_eq!(Point::new(9, 6), v1 + v2);
    assert_eq!(Point::new(-3, 2), v1 - v2);
    assert_eq!(Point::new(6, 8), v1 * 2);
    //assert_eq!(Point::new(6, 8), 2 * v1);
    assert_eq!(Point::new(3, 1), v2 / 2);

    let r = Rect::new(5, 6, 7, 8);
    let m = Margins::new(2, 2, 2, 2);

    assert_eq!(Rect::new(3, 4, 11, 12), r + m);
    assert_eq!(Rect::new(7, 8, 3, 4), r - m);
}
