//! Equirectangular projection.

use std::convert::Infallible;

use boolygon::point::Point;
use geocart::{geographic::Geographic, positive::Positive};
use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{nonzero::NonZero, Projection};

/// The [equirectangular projection](https://en.wikipedia.org/wiki/Equirectangular_projection).
#[derive(Debug)]
pub struct Equirectangular<T> {
    /// The radius of the globe being projected.
    pub radius: NonZero<Positive<T>>,
}

impl<T> Projection<T> for Equirectangular<T>
where
    T: Default + Signed + Float + FloatConst + Euclid,
{
    type Error = Infallible;

    fn project(&self, coordinates: &Geographic<T>) -> Result<Point<T>, Self::Error> {
        let radius = self.radius.into_inner().into_inner();

        Ok(Point {
            x: radius * coordinates.longitude.into_inner(),
            y: radius * coordinates.latitude.into_inner(),
        })
    }

    fn inverse(&self, coordinates: &Point<T>) -> Result<Geographic<T>, Self::Error> {
        let radius = self.radius.into_inner().into_inner();

        Ok(Geographic {
            latitude: (coordinates.y / radius).into(),
            longitude: (coordinates.x / radius).into(),
            ..Default::default()
        })
    }
}
