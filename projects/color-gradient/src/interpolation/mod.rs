use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

type Precision = u16;

#[derive(Clone, Debug)]
pub struct Interpolator {
    keys: BTreeMap<Precision, f32>,
    lhs: f32,
    rhs: f32,
}

impl Interpolator {
    pub fn new(min: f32, max: f32) -> Self {
        Self { keys: Default::default(), lhs: min, rhs: max }
    }
    pub fn head(&self) -> f32 {
        self.keys.get(&Precision::MIN).copied().unwrap_or(self.lhs)
    }
    pub fn tail(&self) -> f32 {
        self.keys.get(&Precision::MAX).copied().unwrap_or(self.rhs)
    }
    pub fn insert(&mut self, key: Precision, value: f32) {
        self.keys.insert(key, value);
    }
    pub fn remove(&mut self, key: Precision) {
        self.keys.remove(&key);
    }
}

impl Interpolator {
    /// Get zero-order interpolation, that is, the first number greater than ratio
    pub fn get_step(&self, key: Precision) -> f32 {
        if key == Precision::MIN {
            self.head()
        }
        else if key == Precision::MAX {
            self.tail()
        }
        else {
            self.keys.range(..=key).next_back().map(|(_, v)| *v).unwrap_or(self.lhs)
        }
    }
    /// Get first-order linear interpolation
    pub fn get_linear(&self, key: Precision) -> f32 {
        if key == Precision::MIN {
            self.head()
        }
        else if key == Precision::MAX {
            self.tail()
        }
        else {
            let s1 = (&Precision::MIN, &self.head());
            let s2 = (&Precision::MAX, &self.tail());
            let (k1, v1) = self.keys.range(..=key).next_back().unwrap_or(s1);
            let (k2, v2) = self.keys.range(key..).next().unwrap_or(s2);
            if k1 == k2 { *v1 } else { v1 + (v2 - v1) * (key - k1) as f32 / (k2 - k1) as f32 }
        }
    }
    pub fn clear(&mut self) {
        self.keys.clear();
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
