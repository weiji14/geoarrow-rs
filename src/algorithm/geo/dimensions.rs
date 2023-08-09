use crate::array::*;
use crate::GeometryArrayTrait;
use arrow2::array::{BooleanArray, MutableBooleanArray};
use arrow2::types::Offset;
use geo::dimensions::HasDimensions as GeoHasDimensions;

/// Operate on the dimensionality of geometries.
pub trait HasDimensions {
    /// Some geometries, like a `MultiPoint`, can have zero coordinates - we call these `empty`.
    ///
    /// Types like `Point` and `Rect`, which have at least one coordinate by construction, can
    /// never be considered empty.
    /// ```
    /// use geo_types::{Point, coord, LineString};
    /// use geo::HasDimensions;
    ///
    /// let line_string = LineString::new(vec![
    ///     coord! { x: 0., y: 0. },
    ///     coord! { x: 10., y: 0. },
    /// ]);
    /// assert!(!line_string.is_empty());
    ///
    /// let empty_line_string: LineString = LineString::new(vec![]);
    /// assert!(empty_line_string.is_empty());
    ///
    /// let point = Point::new(0.0, 0.0);
    /// assert!(!point.is_empty());
    /// ```
    fn is_empty(&self) -> BooleanArray;
}

// Note: this can't (easily) be parameterized in the macro because PointArray is not generic over O
impl HasDimensions for PointArray {
    fn is_empty(&self) -> BooleanArray {
        let mut output_array = MutableBooleanArray::with_capacity(self.len());
        self.iter_geo()
            .for_each(|maybe_g| output_array.push(maybe_g.map(|g| g.is_empty())));
        output_array.into()
    }
}

/// Implementation that iterates over geo objects
macro_rules! iter_geo_impl {
    ($type:ty) => {
        impl<C: CoordBuffer, O: Offset> HasDimensions for $type {
            fn is_empty(&self) -> BooleanArray {
                let mut output_array = MutableBooleanArray::with_capacity(self.len());
                self.iter_geo()
                    .for_each(|maybe_g| output_array.push(maybe_g.map(|g| g.is_empty())));
                output_array.into()
            }
        }
    };
}

iter_geo_impl!(LineStringArray<O>);
iter_geo_impl!(PolygonArray<O>);
iter_geo_impl!(MultiPointArray<O>);
iter_geo_impl!(MultiLineStringArray<O>);
iter_geo_impl!(MultiPolygonArray<O>);
iter_geo_impl!(WKBArray<O>);

impl<C: CoordBuffer, O: Offset> HasDimensions for GeometryArray<O> {
    fn is_empty(&self) -> BooleanArray {
        match self {
            GeometryArray::WKB(arr) => HasDimensions::is_empty(arr),
            GeometryArray::Point(arr) => HasDimensions::is_empty(arr),
            GeometryArray::LineString(arr) => HasDimensions::is_empty(arr),
            GeometryArray::Polygon(arr) => HasDimensions::is_empty(arr),
            GeometryArray::MultiPoint(arr) => HasDimensions::is_empty(arr),
            GeometryArray::MultiLineString(arr) => HasDimensions::is_empty(arr),
            GeometryArray::MultiPolygon(arr) => HasDimensions::is_empty(arr),
            _ => todo!(),
        }
    }
}
