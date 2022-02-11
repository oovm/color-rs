use std::ops::{Add, Mul};
use num_traits::{One, Zero};
use crate::float32::f01;

impl Zero for f01 {
    fn zero() -> Self {
        Self {
            wrapped: 0.0,
        }
    }

    fn is_zero(&self) -> bool {
        self.wrapped.is_zero()
    }
}

impl One for f01 {
    fn one() -> Self {
        Self {
            wrapped: 1.0,
        }
    }
}

impl Add<Self> for f01 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            wrapped: self.wrapped + rhs.wrapped,
        }
    }
}

impl Mul<Self> for f01 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            wrapped: self.wrapped * rhs.wrapped,
        }
    }
}

