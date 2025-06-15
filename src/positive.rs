//! Positive definition.

use std::ops::Add;

use num_traits::{Signed, Zero};

/// A value that is always positive.
///
/// This wrapper extends the [`Positive`](geocart::positive::Positive) type.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Positive<T>(geocart::positive::Positive<T>);

impl<T> From<T> for Positive<T>
where
    T: Signed,
{
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl<T> From<geocart::positive::Positive<T>> for Positive<T> {
    fn from(positive: geocart::positive::Positive<T>) -> Self {
        Self(positive)
    }
}

impl<T> Add<Self> for Positive<T>
where
    T: Signed + Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.into_inner() + rhs.into_inner())
    }
}

impl<T> Zero for Positive<T>
where
    T: Signed + Zero,
{
    fn zero() -> Self {
        Self(T::zero().into())
    }

    fn is_zero(&self) -> bool {
        self.0.as_ref().is_zero()
    }
}

impl<T> Eq for Positive<T> where T: PartialEq {}

impl<T> Positive<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0.into_inner()
    }
}
