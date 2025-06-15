pub mod nonzero;
pub mod positive;

#[cfg(any(feature = "equirectangular", feature = "gall-stereographic"))]
pub mod projection;

use boolygon::point::Point;
use geocart::geographic::Geographic;

/// A projection is a function that maps [`Geographic`] coordinates to [`Point`]s on the plane, and
/// vice-versa.
pub trait Projection<T> {
    /// The type of error that may occur when performing the projection.
    type Error;

    /// Projects the given [`Geographic`] coordinates onto the plane, returning the corresponding
    /// [`Point`].
    fn project(&self, coordinates: &Geographic<T>) -> Result<Point<T>, Self::Error>;

    /// Performs the inverse of the projection, returning the [`Geographic`] coordiantes of the
    /// given [`Point`].
    fn inverse(&self, coordinates: &Point<T>) -> Result<Geographic<T>, Self::Error>;
}
