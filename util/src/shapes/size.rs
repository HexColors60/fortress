use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Rem;

use super::Point2;

///
/// A Size is an area of space with no location.
/// The size of a box, the size of a window, the size of a player.
/// But they don't have a location.
///
#[derive(Debug, Copy, Clone)]
pub struct Size<N: Add + Sub + Mul + Div + Rem + Copy> {
    /// The width of the area.
    pub width: N,

    /// The height of the area.
    pub height: N,
}

impl<N: Add + Sub + Mul + Div + Rem + Copy> Size<N> {
    ///
    /// Trivial constructor.
    ///
    /// Creates a new Size with the width and height given.
    ///
    pub fn new(width: N, height: N) -> Size<N> {
        Size {
            width: width,
            height: height,
        }
    }

    ///
    /// Converts this to a Point2.
    ///
    pub fn to_point2(&self) -> Point2<N> {
        Point2 {
            x: self.width,
            y: self.height,
        }
    }
}

impl<N> PartialEq for Size<N>
where
    N: Add + Sub + Mul + Div + Rem + Copy + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.width == other.width && self.height == other.height
    }
}

impl<N> Add for Size<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Size {
            width: (self.width + other.width),
            height: (self.height + other.height),
        }
    }
}

impl<N> Sub for Size<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Size {
            width: (self.width - other.width),
            height: (self.height - other.height),
        }
    }
}

impl<N> Mul<N> for Size<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn mul(self, other: N) -> Self {
        Size {
            width: (self.width * other),
            height: (self.height * other),
        }
    }
}

impl<N> Div<N> for Size<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    type Output = Self;

    fn div(self, other: N) -> Self {
        Size {
            width: (self.width / other),
            height: (self.height / other),
        }
    }
}

impl<N> Into<Size<N>> for Point2<N>
where
    N: Add<Output = N>
        + Sub<Output = N>
        + Mul<Output = N>
        + Div<Output = N>
        + Rem<Output = N>
        + Copy,
{
    fn into(self) -> Size<N> {
        Size {
            width: self.x,
            height: self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create() {
        assert_eq!(
            Size {
                width: 1,
                height: 5,
            },
            Size {
                width: 1,
                height: 5,
            }
        );
    }

    #[test]
    fn new() {
        assert_eq!(
            Size {
                width: 1,
                height: 5,
            },
            Size::new(1, 5)
        );
    }

    #[test]
    fn to_point2() {
        assert_eq!(Point2 { x: 1, y: 5 }, Size::new(1, 5).to_point2());
    }

    #[test]
    fn add() {
        assert_eq!(
            Size {
                width: 1,
                height: 5,
            } + Size {
                width: 93,
                height: 28,
            },
            Size {
                width: 94,
                height: 33,
            }
        );

        assert_eq!(
            Size {
                width: 50,
                height: 10,
            } + Size {
                width: 100,
                height: 5,
            },
            Size {
                width: 150,
                height: 15,
            }
        );
    }

    #[test]
    fn sub() {
        assert_eq!(
            Size {
                width: 60,
                height: 30,
            } - Size {
                width: 1,
                height: 5,
            },
            Size {
                width: 59,
                height: 25,
            }
        );

        assert_eq!(
            Size {
                width: 50,
                height: 10,
            } - Size {
                width: 100,
                height: 5,
            },
            Size {
                width: -50,
                height: 5,
            }
        );
    }
}