use crate::{RGB, RGBA, RGBA32};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<RGB> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RGB {
        RGB { r: rng.gen(), g: rng.gen(), b: rng.gen() }
    }
}

impl Distribution<RGBA> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RGBA {
        RGBA { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: rng.gen() }
    }
}

impl Distribution<RGBA32> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RGBA32 {
        RGBA32 {
            r: rng.gen_range(0f32..1f32),
            g: rng.gen_range(0f32..1f32),
            b: rng.gen_range(0f32..1f32),
            a: rng.gen_range(0f32..1f32),
        }
    }
}
