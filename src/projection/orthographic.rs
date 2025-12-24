//! Orthographic map projection.

use boolygon::point::Point;
use geocart::geographic::Geographic;
use num_traits::{Euclid, Float, FloatConst, Signed};

use crate::{Projection, nonzero::NonZero, positive::Positive};

use super::Error;

/// The [Orthographic map projection](https://en.wikipedia.org/wiki/Orthographic_map_projection).
pub struct Orthographic<T> {
    /// The radius of the globe being projected.
    pub radius: NonZero<Positive<T>>,
    /// The central point of the hemisphere.
    pub origin: Geographic<T>,
}

impl<T> Projection<T> for Orthographic<T>
where
    T: Default + Signed + Float + FloatConst + Euclid,
{
    type Error = Error;

    fn project(&self, coordinates: &Geographic<T>) -> Result<Point<T>, Self::Error> {
        let (sin_longitude, cos_longitude) = {
            let longitude = coordinates.longitude.into_inner() - self.origin.longitude.into_inner();
            (longitude.sin(), longitude.cos())
        };

        let (sin_origin_lat, cos_origin_lat) = {
            let origin_lat = self.origin.latitude.into_inner();
            (origin_lat.sin(), origin_lat.cos())
        };

        let (sin_latitude, cos_latitude) = {
            let latitude = coordinates.latitude.into_inner();
            (latitude.sin(), latitude.cos())
        };

        if (sin_origin_lat * sin_latitude + cos_origin_lat * cos_latitude * cos_longitude)
            .is_sign_negative()
        {
            return Err(Error::Unprojectable(
                "the coordinates do not belong to the hemisphere",
            ));
        }

        let radius = self.radius.into_inner().into_inner();

        Ok(Point {
            x: radius * cos_latitude * sin_longitude,
            y: (cos_origin_lat * sin_latitude - sin_origin_lat * cos_latitude * cos_longitude)
                * radius,
        })
    }

    fn inverse(&self, coordinates: &Point<T>) -> Result<Geographic<T>, Self::Error> {
        let p = (coordinates.x.powi(2) + coordinates.y.powi(2)).sqrt();
        let radius = self.radius.into_inner().into_inner();

        if p > radius {
            return Err(Error::Unprojectable(
                "the point does not belong to the hemisphere",
            ));
        }

        let (sin_c, cos_c) = {
            let c = (p / radius).asin();
            (c.sin(), c.cos())
        };

        let origin_lon = self.origin.longitude.into_inner();

        let (sin_origin_lat, cos_origin_lat) = {
            let origin_lat = self.origin.latitude.into_inner();
            (origin_lat.sin(), origin_lat.cos())
        };

        Ok(Geographic {
            longitude: ((coordinates.x * sin_c)
                .atan2(p * cos_c * cos_origin_lat - coordinates.y * sin_c * sin_origin_lat)
                + origin_lon)
                .into(),
            latitude: (cos_c * sin_origin_lat + (coordinates.y * sin_c * cos_origin_lat / p))
                .asin()
                .into(),
            ..Default::default()
        })
    }
}
