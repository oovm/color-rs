use super::*;

impl Eq for f01 {}

impl PartialEq<Self> for f01 {
    fn eq(&self, other: &Self) -> bool {
        self.wrapped == other.wrapped
    }
}

impl PartialEq<f32> for f01 {
    fn eq(&self, other: &f32) -> bool {
        self.wrapped.eq(other)
    }
}

impl PartialEq<f64> for f01 {
    fn eq(&self, other: &f64) -> bool {
        (self.wrapped as f64).eq(other)
    }
}


impl Ord for f01 {
    fn cmp(&self, other: &Self) -> Ordering {
        unsafe {
            self.wrapped.partial_cmp(&other.wrapped).unwrap_unchecked()
        }
    }
}


impl PartialOrd<Self> for f01 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.wrapped.partial_cmp(&other.wrapped)
    }
}

impl PartialOrd<f32> for f01 {
    fn partial_cmp(&self, other: &f32) -> Option<Ordering> {
        self.wrapped.partial_cmp(other)
    }
}


impl PartialOrd<f64> for f01 {
    fn partial_cmp(&self, other: &f64) -> Option<Ordering> {
        (self.wrapped as f64).partial_cmp(other)
    }
}