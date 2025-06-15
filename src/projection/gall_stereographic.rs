//! Gall Stereographic projection.

use std::convert::Infallible;

use boolygon::point::Point;
use geocart::geographic::Geographic;
use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{nonzero::NonZero, positive::Positive, Projection};

/// The [Gall Stereographic projection](https://en.wikipedia.org/wiki/Gall_stereographic_projection).
pub struct GallStereographic<T> {
    /// The radius of the globe being projected.
    pub radius: NonZero<Positive<T>>,
}

impl<T> Projection<T> for GallStereographic<T>
where
    T: Default + PartialOrd + Signed + Float + FloatConst + Euclid,
{
    type Error = Infallible;

    fn project(&self, coordinates: &Geographic<T>) -> Result<Point<T>, Self::Error> {
        let radius = self.radius.into_inner().into_inner();
        let two = T::one() + T::one();

        Ok(Point {
            x: radius * coordinates.longitude.into_inner() / T::SQRT_2(),
            y: radius
                * (T::one() + T::SQRT_2() / two)
                * (coordinates.latitude.into_inner() / two).tan(),
        })
    }

    fn inverse(&self, coordinates: &Point<T>) -> Result<Geographic<T>, Self::Error> {
        let radius = self.radius.into_inner().into_inner();
        let two = T::one() + T::one();

        Ok(Geographic {
            latitude: (two * (coordinates.y / (radius * (T::one() + T::SQRT_2() / two))).atan())
                .into(),
            longitude: (coordinates.x * T::SQRT_2() / radius).into(),
            ..Default::default()
        })
    }
}
