use super::*;

mod display;

impl LUMA32 {}

impl Default for LUMA32 {
    fn default() -> Self {
        Self { l: 0.0, u: 0.0, m: 0.0, a: 1.0 }
    }
}
