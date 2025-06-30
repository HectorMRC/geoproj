//! The [`Name`] type definition.

use std::{marker::PhantomData, str::FromStr};

use serde::{Deserialize, Serialize};

/// The error type for the implementation of [`FromStr`] for [`Name`].
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

    fn from_str(value: &str) -> Result<Self, Error> {
        if value.contains(|c| ['\n', '\r'].contains(&c)) {
            return Err(Error::MultiLine);
        }

        Ok(Self {
            value: value.to_string(),
            _type: PhantomData,
        })
    }
}
