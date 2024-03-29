use std::collections::BTreeMap;

type Resolution = u16;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Interpolator {
    keys: BTreeMap<Resolution, f32>,
    lhs: f32,
    rhs: f32,
}

impl Interpolator {
    pub fn new(min: f32, max: f32) -> Self {
        Self { keys: Default::default(), lhs: min, rhs: max }
    }
    pub fn head(&self) -> f32 {
        self.keys.get(&Resolution::MIN).copied().unwrap_or(self.lhs)
    }
    pub fn tail(&self) -> f32 {
        self.keys.get(&Resolution::MAX).copied().unwrap_or(self.rhs)
    }
    pub fn insert(&mut self, key: Resolution, value: f32) {
        self.keys.insert(key, value);
    }
    pub fn remove(&mut self, key: Resolution) {
        self.keys.remove(&key);
    }
}

impl Interpolator {
    /// Get zero-order interpolation, that is, the first number greater than ratio
    pub fn get_step(&self, key: Resolution) -> f32 {
        if key == Resolution::MIN {
            self.head()
        }
        else if key == Resolution::MAX {
            self.tail()
        }
        else {
            self.keys.range(..=key).next_back().map(|(_, v)| *v).unwrap_or(self.lhs)
        }
    }
    /// Get first-order linear interpolation
    pub fn get_linear(&self, key: Resolution) -> f32 {
        if key == Resolution::MIN {
            self.head()
        }
        else if key == Resolution::MAX {
            self.tail()
        }
        else {
            let s1 = (&Resolution::MIN, &self.head());
            let s2 = (&Resolution::MAX, &self.tail());
            let (k1, v1) = self.keys.range(..=key).next_back().unwrap_or(s1);
            let (k2, v2) = self.keys.range(key..).next().unwrap_or(s2);
            if k1 == k2 { *v1 } else { v1 + (v2 - v1) * (key - k1) as f32 / (k2 - k1) as f32 }
        }
    }
    pub fn clear(&mut self) {
        self.keys.clear();
    }
}
