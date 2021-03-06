use num_types::FromClamped;
use num_types::internal::SaturatingAddT;
use std::convert::From;

use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::DivAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Rem;
use std::ops::Sub;
use std::ops::SubAssign;

use num_types::Num;

use super::Point;
use super::Size;

/// A rectangle.
///
/// It has a position, and a size. It's generic parameter allows
/// you to use any numerical type for it. Integers, floats, etc.
/// 
#[derive(Copy, Clone, Debug)]
pub struct Rect<
    N: Add
        + Sub
        + Mul
        + Div
        + Rem
        + fmt::Display
        + Copy
        + AddAssign
        + DivAssign
        + MulAssign
        + SubAssign,
> {
    /// It's x location.
    pub x: N,

    /// It's y location.
    pub y: N,

    /// The width of this rectangle.
    /// It can have positive or negative size.
    pub width: N,

    /// The height of this rectangle.
    /// It can have positive or negative size.
    pub height: N,
}

impl<
    N: Add
        + Sub
        + Mul
        + Div
        + Rem
        + fmt::Display
        + Copy
        + From<u8>
        + AddAssign
        + DivAssign
        + MulAssign
        + SubAssign,
> Rect<N>
{
    /// Trivial constructor.
    ///
    /// Creates a new Rect with the size given.
    pub const fn new(
        x: N,
        y: N,
        w: N,
        h: N,
    ) -> Self {
        Rect {
            x: x,
            y: y,

            width: w,
            height: h,
        }
    }

    /// Returns the x/y section of this rectangle on it's own.
    pub const fn point(&self) -> Point<N> {
        Point {
            x: self.x,
            y: self.y,
        }
    }

    /// Returns the width/height section of this rectangle on it's own.
    pub const fn size(&self) -> Size<N> {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl<N: Num<N>> fmt::Display for Rect<N> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.width, self.height)
    }
}

impl<N: Num<N> + PartialOrd> Rect<N> {
    /// Returns true if the point is within the size.
    /// The check is exclusive of the size. i.e. `x < width`.
    pub fn contains(
        &self,
        point: Point<N>,
    ) -> bool {
        point.x >= self.x && point.y >= self.y && point.x < self.width && point.y < self.height
    }
}

impl<N: Num<N> + PartialOrd + SaturatingAddT> Rect<N> {
    /// Returns a rectangle that only contains the overlapping section.
    #[cfg_attr(feature = "cargo-clippy", allow(if_same_then_else))]
    pub fn overlap_of(
        &self,
        other: Self,
    ) -> Option<Self> {
        // We are far on the left.
        if self.x.saturating_add_t(self.width) < other.x {
            None
        // They are far on the left of us.
        } else if other.x.saturating_add_t(other.width) < self.x {
            None
        // We are far above.
        } else if self.y.saturating_add_t(self.height) < other.y {
            None
        // They are far above us.
        } else if other.y.saturating_add_t(other.height) < self.y {
            None
        // We have an overlap!
        } else {
            Some(Rect {
                x: if self.x < other.x {
                    other.x
                } else {
                    self.x
                },
                y: if self.y < other.y {
                    other.y
                } else {
                    self.y
                },
                width: if self.width < other.width {
                    self.width
                } else {
                    other.width
                },
                height: if self.height < other.height {
                    self.height
                } else {
                    other.height
                },
            })
        }
    }
}

impl<N: Num<N> + From<u8>> Rect<N> {
    /// Divides the rectangles size by the amount given.
    /// This is done around the centre of the rectangle.
    ///
    /// So this affects both the x/y values, as well as the width and height.
    pub fn divide_around_centre(
        &self,
        divider: N,
    ) -> Self {
        Rect {
            x: self.x + (self.width / divider) / N::from(2),
            y: self.y + (self.height / divider) / N::from(2),
            width: self.width / divider,
            height: self.height / divider,
        }
    }
}

impl<N: Num<N>> Add<Point<N>> for Rect<N> {
    type Output = Self;

    fn add(
        self,
        other: Point<N>,
    ) -> Self {
        Rect {
            x: (self.x + other.x),
            y: (self.y + other.y),
            width: self.width,
            height: self.height,
        }
    }
}

impl<N: Num<N>> Add<Size<N>> for Rect<N> {
    type Output = Self;

    fn add(
        self,
        other: Size<N>,
    ) -> Self {
        Rect {
            x: self.x,
            y: self.y,
            width: (self.width + other.width),
            height: (self.height + other.height),
        }
    }
}

impl<N: Num<N>> Sub<Point<N>> for Rect<N> {
    type Output = Self;

    fn sub(
        self,
        other: Point<N>,
    ) -> Self {
        Rect {
            x: (self.x - other.x),
            y: (self.y - other.y),
            width: self.width,
            height: self.height,
        }
    }
}

impl<N: Num<N>> Sub<Size<N>> for Rect<N> {
    type Output = Self;

    fn sub(
        self,
        other: Size<N>,
    ) -> Self {
        Rect {
            x: self.x,
            y: self.y,
            width: (self.width - other.width),
            height: (self.height - other.height),
        }
    }
}

impl<N: Num<N>> PartialEq for Rect<N> {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.x == other.x && self.y == other.y && self.width == other.width
            && self.height == other.height
    }
}

impl<U: Num<U>> Rect<U> {
    /// This is to allow creating a new Rect, with a new type, from the type given.
    /// i.e. `Rect::new(1 as u8, 1 as u8)::to::<u32>()`
    pub fn to<F: Num<F> + From<U>>(&self) -> Rect<F> {
        Rect {
            x: F::from(self.x),
            y: F::from(self.y),
            width: F::from(self.width),
            height: F::from(self.height),
        }
    }
}

impl<A: Num<A>> Rect<A> {
    /// Converts to a new type. If the current values don't fit in the new type,
    /// then it'll be clamped between min and max.
    /// i.e. `Rect::new(1 as i16, 1 as i16)::to::<u16>()`
    pub fn to_clamped<B: Num<B> + FromClamped<A>>(&self) -> Rect<B> {
        Rect {
            x: B::from_clamped(self.x),
            y: B::from_clamped(self.y),
            width: B::from_clamped(self.width),
            height: B::from_clamped(self.height),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn divide_around_centre() {
        assert_eq!(Rect::new(1, 50, 2, 60), Rect::new(0, 20, 4, 120).divide_around_centre(2),);
    }
}
