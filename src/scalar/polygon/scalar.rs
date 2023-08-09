use crate::algorithm::native::bounding_rect::bounding_rect_polygon;
use crate::algorithm::native::eq::polygon_eq;
use crate::array::polygon::iterator::PolygonInteriorIterator;
use crate::array::polygon::parse_polygon;
use crate::array::CoordBuffer;
use crate::geo_traits::PolygonTrait;
use crate::scalar::LineString;
use crate::trait_::GeometryScalarTrait;
use arrow2::offset::OffsetsBuffer;
use arrow2::types::Offset;
use rstar::{RTreeObject, AABB};
use std::borrow::Cow;

/// An Arrow equivalent of a Polygon
#[derive(Debug, Clone)]
pub struct Polygon<'a, O: Offset> {
    pub coords: Cow<'a, CoordBuffer>,

    /// Offsets into the ring array where each geometry starts
    pub geom_offsets: Cow<'a, OffsetsBuffer<O>>,

    /// Offsets into the coordinate array where each ring starts
    pub ring_offsets: Cow<'a, OffsetsBuffer<O>>,

    pub geom_index: usize,
}

impl<'a, O: Offset> Polygon<'a, O> {
    pub fn new(
        coords: Cow<'a, CoordBuffer>,
        geom_offsets: Cow<'a, OffsetsBuffer<O>>,
        ring_offsets: Cow<'a, OffsetsBuffer<O>>,
        geom_index: usize,
    ) -> Self {
        Self {
            coords,
            geom_offsets,
            ring_offsets,
            geom_index,
        }
    }

    pub fn new_borrowed(
        coords: &'a CoordBuffer,
        geom_offsets: &'a OffsetsBuffer<O>,
        ring_offsets: &'a OffsetsBuffer<O>,
        geom_index: usize,
    ) -> Self {
        Self {
            coords: Cow::Borrowed(coords),
            geom_offsets: Cow::Borrowed(geom_offsets),
            ring_offsets: Cow::Borrowed(ring_offsets),
            geom_index,
        }
    }

    pub fn new_owned(
        coords: CoordBuffer,
        geom_offsets: OffsetsBuffer<O>,
        ring_offsets: OffsetsBuffer<O>,
        geom_index: usize,
    ) -> Self {
        Self {
            coords: Cow::Owned(coords),
            geom_offsets: Cow::Owned(geom_offsets),
            ring_offsets: Cow::Owned(ring_offsets),
            geom_index,
        }
    }
}

impl<'a, O: Offset> GeometryScalarTrait<'a> for Polygon<'a, O> {
    type ScalarGeo = geo::Polygon;

    fn to_geo(&self) -> Self::ScalarGeo {
        self.into()
    }
}

impl<'a, O: Offset> PolygonTrait<'a> for Polygon<'a, O> {
    type T = f64;
    type ItemType = LineString<'a, O>;
    type Iter = PolygonInteriorIterator<'a, O>;

    fn exterior(&self) -> Option<Self::ItemType> {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        if start == end {
            None
        } else {
            Some(LineString::new(
                self.coords.clone(),
                self.ring_offsets.clone(),
                start,
            ))
        }
    }

    fn interiors(&'a self) -> Self::Iter {
        PolygonInteriorIterator::new(self)
    }

    fn num_interiors(&self) -> usize {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        end - start - 1
    }

    fn interior(&self, i: usize) -> Option<Self::ItemType> {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        if i > (end - start - 1) {
            return None;
        }

        Some(LineString::new(
            self.coords.clone(),
            self.ring_offsets.clone(),
            start + 1 + i,
        ))
    }
}

impl<'a, O: Offset> PolygonTrait<'a> for &Polygon<'a, O> {
    type T = f64;
    type ItemType = LineString<'a, O>;
    type Iter = PolygonInteriorIterator<'a, O>;

    fn exterior(&self) -> Option<Self::ItemType> {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        if start == end {
            None
        } else {
            Some(LineString::new(
                self.coords.clone(),
                self.ring_offsets.clone(),
                start,
            ))
        }
    }

    fn interiors(&'a self) -> Self::Iter {
        PolygonInteriorIterator::new(self)
    }

    fn num_interiors(&self) -> usize {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        end - start - 1
    }

    fn interior(&self, i: usize) -> Option<Self::ItemType> {
        let (start, end) = self.geom_offsets.start_end(self.geom_index);
        if i > (end - start - 1) {
            return None;
        }

        Some(LineString::new(
            self.coords.clone(),
            self.ring_offsets.clone(),
            start + 1 + i,
        ))
    }
}

impl<C: CoordBuffer, O: Offset> From<Polygon<'_, O>> for geo::Polygon {
    fn from(value: Polygon<'_, O>) -> Self {
        (&value).into()
    }
}

impl<C: CoordBuffer, O: Offset> From<&Polygon<'_, O>> for geo::Polygon {
    fn from(value: &Polygon<'_, O>) -> Self {
        parse_polygon(
            value.coords.clone(),
            value.geom_offsets.clone(),
            value.ring_offsets.clone(),
            value.geom_index,
        )
    }
}

impl<C: CoordBuffer, O: Offset> From<Polygon<'_, O>> for geo::Geometry {
    fn from(value: Polygon<'_, O>) -> Self {
        geo::Geometry::Polygon(value.into())
    }
}

impl<C: CoordBuffer, O: Offset> RTreeObject for Polygon<'_, O> {
    type Envelope = AABB<[f64; 2]>;

    fn envelope(&self) -> Self::Envelope {
        let (lower, upper) = bounding_rect_polygon(self);
        AABB::from_corners(lower, upper)
    }
}

impl<C: CoordBuffer, O: Offset> PartialEq for Polygon<'_, O> {
    fn eq(&self, other: &Self) -> bool {
        polygon_eq(self, other)
    }
}

#[cfg(test)]
mod test {
    use crate::array::PolygonArray;
    use crate::test::polygon::{p0, p1};
    use crate::GeometryArrayTrait;

    /// Test Eq where the current index is true but another index is false
    #[test]
    fn test_eq_other_index_false() {
        let arr1: PolygonArray<i32> = vec![p0(), p1()].into();
        let arr2: PolygonArray<i32> = vec![p0(), p0()].into();

        assert_eq!(arr1.value(0), arr2.value(0));
        assert_ne!(arr1.value(1), arr2.value(1));
    }
}
