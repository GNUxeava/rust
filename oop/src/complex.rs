use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Complex<T> {
    // Real number
    pub r: T,
    // Imaginary number
    pub i: T,
}

impl<T: Copy + Clone + Add<Output = T> + Sub<Output = T>>
Complex<T> {
    pub fn new(r: T, i: T) -> Self {
        Self{
            r, i
        }
    }

    pub fn add(&mut self, other: &Self) {
        self.r = self.r + other.r;
        self.i = self.i + other.i;
    }

    pub fn sub(&mut self, other: &Self) {
        self.r = self.r - other.r;
        self.i = self.i - other.i;
    }
}

impl<T: Display> Display for Complex<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} + {}i", self.r, self.i)
    }
}

impl<T> From<(T, T)> for Complex<T> {
    fn from(t: (T, T)) -> Self {
        Self {
            r: t.0,
            i: t.1
        }
    }
}
