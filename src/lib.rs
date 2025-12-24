pub mod name;
pub mod nonzero;
pub mod positive;
pub mod projection;
#[cfg(feature = "svg")]
pub mod svg;

use boolygon::point::Point;
use geocart::geographic::Geographic;
use name::Name;
use nonzero::NonZero;
use positive::Positive;

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

/// A geographic polygon.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Polygon<T> {
    /// The ordered list of [`Geographic`] points describing a polygon.
    vertices: Vec<Geographic<T>>,
}

/// A combination of disjoint [`Polygon`]s.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Shape<T> {
    /// The name of this shape.
    name: Name<Self>,
    /// The list of non-crossing [`Polygon`]s.
    polygons: Vec<Polygon<T>>,
}

/// An arbitrary globe whose surface is composed of [`Shape`]s.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Globe<T> {
    /// The name of this globe.
    name: Name<Self>,
    /// The radius of this globe.
    radius: NonZero<Positive<T>>,
    /// The list of shapes on the globe.
    shapes: Vec<Shape<T>>,
}
