use boolygon::point::Point;
use geocart::geographic::Geographic;
use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{nonzero::NonZero, positive::Positive, Projection};

#[derive(Debug, thiserror::Error)]
pub enum OrthographicError {
    #[error("the point does not belongs to the hemisphere")]
    OutOfHemisphere,
    #[error("the point does not belongs to the plane")]
    OutOfPlane,
}

/// The [Orthographic map projection](https://en.wikipedia.org/wiki/Orthographic_map_projection).
pub struct Orthographic<T> {
    /// The radius of the globe being projected.
    pub radius: NonZero<Positive<T>>,
    /// The central point of the hemisphere to project.
    pub origin: Geographic<T>,
}

impl<T> Projection<T> for Orthographic<T>
where
    T: Default + Signed + Float + FloatConst + Euclid,
{
    type Error = OrthographicError;

    fn project(&self, coordinates: &Geographic<T>) -> Result<Point<T>, Self::Error> {
        let longitude = coordinates.longitude.into_inner() - self.origin.longitude.into_inner();
        let cos_c = self.origin.latitude.into_inner().sin()
            * coordinates.latitude.into_inner().sin()
            + self.origin.latitude.into_inner().cos()
                * coordinates.latitude.into_inner().cos()
                * longitude.cos();

        if cos_c.is_sign_negative() {
            return Err(OrthographicError::OutOfHemisphere);
        }

        let radius = self.radius.into_inner().into_inner();
        let latitude_0 = self.origin.latitude.into_inner();
        let latitude = coordinates.latitude.into_inner();

        Ok(Point {
            x: radius * latitude.cos() * longitude.sin(),
            y: (latitude_0.cos() * latitude.sin()
                - latitude_0.sin() * latitude.cos() * longitude.cos())
                * radius,
        })
    }

    fn inverse(&self, coordinates: &Point<T>) -> Result<Geographic<T>, Self::Error> {
        let p = (coordinates.x.powi(2) + coordinates.y.powi(2)).sqrt();
        let radius = self.radius.into_inner().into_inner();

        if p > radius {
            return Err(OrthographicError::OutOfPlane);
        }

        let c = (p / radius).asin();
        let cos_c = c.cos();
        let sin_c = c.sin();

        let longitude_0 = self.origin.longitude.into_inner();
        let latitude_0 = self.origin.latitude.into_inner();
        let cos_latitude_0 = latitude_0.cos();
        let sin_latitude_0 = latitude_0.sin();

        Ok(Geographic {
            longitude: ((coordinates.x * sin_c)
                .atan2(p * cos_c * cos_latitude_0 - coordinates.y * sin_c * sin_latitude_0)
                + longitude_0)
                .into(),
            latitude: (cos_c * sin_latitude_0 + (coordinates.y * sin_c * cos_latitude_0 / p))
                .asin()
                .into(),
            ..Default::default()
        })
    }
}
