//! Serde implementation.

pub mod name;

use geocart::geographic::Geographic;
use name::Name;

pub enum Shape<T> {
    Polygon(Vec<Geographic<T>>),
    Path(Vec<Geographic<T>>),
    Point(Geographic<T>),
}

pub struct Layer<T> {
    name: Name<Self>,
    shape: Shape<T>,
}
