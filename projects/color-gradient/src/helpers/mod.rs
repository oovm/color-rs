//! todo
#[cfg(feature = "image")]
mod sampler;

#[cfg(feature = "image")]
pub use self::sampler::GradientSampler;
