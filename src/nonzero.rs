//! Arbitrary non-zero type.

use num_traits::Zero;

/// A type that may take any value but zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NonZero<T>(T);

impl<T> NonZero<T>
where
    T: Zero,
{
    /// Returns a new instance if, and only if, the given inner value is not zero.
    pub fn new(value: T) -> Option<Self> {
        if value.is_zero() {
            return None;
        }

        Some(Self(value))
    }
}

impl<T> NonZero<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}
