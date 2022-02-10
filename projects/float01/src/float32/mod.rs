use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

mod arith;
mod convert;
mod display;

#[cfg(feature = "serde")]
mod ser;
#[cfg(feature = "serde")]
mod der;
#[cfg(feature = "num-traits")]
mod num;

/// A float in the range [0.0, 1.0].
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, )]
pub struct f01 {
    wrapped: f32,
}


impl f01 {
    pub fn new(ranged: f32) -> Self {
        Self { wrapped: ranged.max(0.0).min(1.0) }
    }
}

impl Default for f01 {
    fn default() -> Self {
        Self { wrapped: 0.0 }
    }
}

impl Hash for f01 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.wrapped.to_bits().hash(state);
    }
}