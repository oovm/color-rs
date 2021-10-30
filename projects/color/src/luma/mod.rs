use std::fmt::{Debug, Formatter};

mod luma32;

#[derive(Clone, Copy, PartialEq, PartialOrd)]
pub struct LUMA32 {
    pub l: f32,
    pub u: f32,
    pub m: f32,
    pub a: f32,
}
