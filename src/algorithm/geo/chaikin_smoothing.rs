use crate::array::*;
use crate::chunked_array::*;
use arrow_array::OffsetSizeTrait;
use geo::ChaikinSmoothing as _ChaikinSmoothing;

/// Smoothen `LineString`, `Polygon`, `MultiLineString` and `MultiPolygon` using Chaikins algorithm.
///
/// [Chaikins smoothing algorithm](http://www.idav.ucdavis.edu/education/CAGDNotes/Chaikins-Algorithm/Chaikins-Algorithm.html)
///
/// Each iteration of the smoothing doubles the number of vertices of the geometry, so in some
/// cases it may make sense to apply a simplification afterwards to remove insignificant
/// coordinates.
///
/// This implementation preserves the start and end vertices of an open linestring and
/// smoothes the corner between start and end of a closed linestring.
pub trait ChaikinSmoothing {
    /// create a new geometry with the Chaikin smoothing being
    /// applied `n_iterations` times.
    fn chaikin_smoothing(&self, n_iterations: u32) -> Self;
}

/// Implementation that iterates over geo objects
macro_rules! iter_geo_impl {
    ($type:ty, $geo_type:ty) => {
        impl<O: OffsetSizeTrait> ChaikinSmoothing for $type {
            fn chaikin_smoothing(&self, n_iterations: u32) -> Self {
                let output_geoms: Vec<Option<$geo_type>> = self
                    .iter_geo()
                    .map(|maybe_g| {
                        maybe_g.map(|geom| geom.chaikin_smoothing(n_iterations.try_into().unwrap()))
                    })
                    .collect();

                output_geoms.into()
            }
        }
    };
}

iter_geo_impl!(LineStringArray<O>, geo::LineString);
iter_geo_impl!(PolygonArray<O>, geo::Polygon);
iter_geo_impl!(MultiLineStringArray<O>, geo::MultiLineString);
iter_geo_impl!(MultiPolygonArray<O>, geo::MultiPolygon);

macro_rules! impl_chunked {
    ($chunked_array:ty) => {
        impl<O: OffsetSizeTrait> ChaikinSmoothing for $chunked_array {
            fn chaikin_smoothing(&self, n_iterations: u32) -> Self {
                self.map(|chunk| chunk.chaikin_smoothing(n_iterations.into()))
                    .try_into()
                    .unwrap()
            }
        }
    };
}

impl_chunked!(ChunkedLineStringArray<O>);
impl_chunked!(ChunkedPolygonArray<O>);
impl_chunked!(ChunkedMultiLineStringArray<O>);
impl_chunked!(ChunkedMultiPolygonArray<O>);