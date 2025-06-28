//! The [`Name`] type definition.

use std::{marker::PhantomData, str::FromStr};

use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum Error {
    #[error("a name cannot be empty")]
    Empty,
    #[error("a name cannot contain more than one line")]
    MultiLine,
}

/// A single-line string that identifies instances of `T`.
#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Name<T> {
    value: String,
    #[serde(skip)]
    _type: PhantomData<T>,
}

impl<T> FromStr for Name<T> {
    type Err = Error;

    /// A name must consist of a single line string.
    fn from_str(value: &str) -> Result<Self> {
        let is_invalid_char = |c: char| -> bool {
            const INVALID_CHARS: [char; 2] = ['\n', '\r'];
            INVALID_CHARS.contains(&c)
        };

        if value.contains(is_invalid_char) {
            return Err(Error::MultiLine);
        }

        Ok(Self {
            value: value.to_string(),
            _type: PhantomData,
        })
    }
}
