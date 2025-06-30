//! Serde implementation.

pub mod name;

use geocart::geographic::Geographic;
use name::Name;

use crate::{nonzero::NonZero, positive::Positive};

/// A geographic polygon.
pub struct Polygon<T> {
    /// The ordered list of [`Geographic`] points describing a polygon.
    vertices: Vec<Geographic<T>>,
}

/// A combination of disjoint [`Polygon`]s.
pub struct Shape<T> {
    /// The list of non-crossing [`Polygon`]s.
    polygons: Vec<Polygon<T>>,
}

/// An arbitrary globe whose surface is composed of [`Shape`]s.
pub struct Globe<T> {
    /// The name of this globe.
    name: Name<Self>,
    /// The radius of this globe.
    radius: NonZero<Positive<T>>,
    /// The list of shapes on the globe.
    shapes: Vec<Shape<T>>,
}
