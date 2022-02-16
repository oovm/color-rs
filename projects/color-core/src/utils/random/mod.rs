use crate::{RGB8, RGBA32, RGBA8};
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};

impl Distribution<RGB8> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RGB8 {
        RGB8 { r: rng.gen(), g: rng.gen(), b: rng.gen() }
    }
}

impl Distribution<RGBA8> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> RGBA8 {
        RGBA8 { r: rng.gen(), g: rng.gen(), b: rng.gen(), a: rng.gen() }
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
