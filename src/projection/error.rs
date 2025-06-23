/// The most trivial error that may be returned by a projection.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("unprojectable: {0}")]
    Unprojectable(&'static str),
}

impl<T> From<Error> for Result<T, Error> {
    fn from(error: Error) -> Self {
        Err(error)
    }
}
