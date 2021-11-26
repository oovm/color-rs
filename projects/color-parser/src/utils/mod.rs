use nom::{
    error::{Error, ErrorKind},
    Err, IResult,
};

pub type ErrorMessage = Err<Error<&'static str>>;

pub fn nom_error<T>(kind: ErrorKind, input: &'static str) -> Result<T, Err<Error<&'static str>>> {
    Err(Err::Error(Error::new(input, kind)))
}

pub fn clamp(value: f32) -> f32 {
    value.max(0.).min(1.)
}
