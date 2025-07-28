use serde::ser::{self, Impossible, Serialize};

/// A SVG serializer.
pub struct Serializer<W> {
    writter: W,
}

//impl<W> serde::ser::Serializer for Serializer<W> {}
