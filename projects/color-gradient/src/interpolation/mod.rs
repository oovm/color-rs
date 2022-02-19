use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
pub struct Interpolator {
    keys: BTreeMap<u8, f32>,
    lhs: f32,
    rhs: f32,
}

impl Interpolator {
    pub fn new(min: f32, max: f32) -> Self {
        Self { keys: Default::default(), lhs: min, rhs: max }
    }
    pub fn head(&self) -> f32 {
        self.keys.get(&0).copied().unwrap_or(self.lhs)
    }
    pub fn tail(&self) -> f32 {
        self.keys.get(&255).copied().unwrap_or(self.rhs)
    }
    pub fn insert(&mut self, key: u8, value: f32) {
        self.keys.insert(key, value);
    }
    pub fn remove(&mut self, key: u8) {
        self.keys.remove(&key);
    }
}

impl Interpolator {
    /// Get zero-order interpolation, that is, the first number greater than ratio
    pub fn get_step(&self, key: u8) -> f32 {
        if key == 0 {
            self.head()
        }
        else if key == 255 {
            self.tail()
        }
        else {
            self.keys.range(..=key).next_back().map(|(_, v)| *v).unwrap_or(self.lhs)
        }
    }
    /// Get first-order linear interpolation
    pub fn get_linear(&self, key: u8) -> f32 {
        if key == 0 {
            self.head()
        }
        else if key == 255 {
            self.tail()
        }
        else {
            let (k1, v1) = self.keys.range(..=key).next_back().unwrap_or((&0, &self.lhs));
            let (k2, v2) = self.keys.range(key..).next().unwrap_or((&255, &self.rhs));
            let ratio = (key - k1) as f32 / (k2 - k1) as f32;
            v1 + (v2 - v1) * ratio
        }
    }
}

#[test]
fn test_step() {
    let mut map = Interpolator::new(0.0, 1.0);
    assert_eq!(map.get_step(0), 0.0);
    assert_eq!(map.get_step(128), 0.0);
    assert_eq!(map.get_step(255), 1.0);
    map.keys.insert(0, 0.1);
    assert_eq!(map.get_step(0), 0.1);
    assert_eq!(map.get_step(128), 0.1);
    assert_eq!(map.get_step(255), 1.0);
    map.keys.insert(255, 0.9);
    assert_eq!(map.get_step(0), 0.1);
    assert_eq!(map.get_step(128), 0.1);
    assert_eq!(map.get_step(255), 0.9);
    map.keys.insert(64, 0.5);
    assert_eq!(map.get_step(0), 0.1);
    assert_eq!(map.get_step(128), 0.5);
    assert_eq!(map.get_step(255), 0.9);
}

#[test]
fn test_linear() {
    let mut map = Interpolator::new(0.0, 1.0);
    assert_eq!(map.get_linear(0), 0.0);
    assert_eq!(map.get_linear(128), 0.5019608);
    assert_eq!(map.get_linear(255), 1.0);
    map.keys.insert(0, 0.1);
    assert_eq!(map.get_linear(0), 0.1);
    assert_eq!(map.get_linear(128), 0.5517647);
    assert_eq!(map.get_linear(255), 1.0);
    map.keys.insert(255, 0.9);
    assert_eq!(map.get_linear(0), 0.1);
    assert_eq!(map.get_linear(128), 0.5015686);
    assert_eq!(map.get_linear(255), 0.9);
    map.keys.insert(64, 0.5);
    assert_eq!(map.get_linear(0), 0.1);
    assert_eq!(map.get_linear(128), 0.6340314);
    assert_eq!(map.get_linear(255), 0.9);
}
