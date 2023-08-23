use geo::{
    CoordNum, Geometry, GeometryCollection, LineString, MultiLineString, MultiPoint, MultiPolygon,
    Point, Polygon, Rect,
};

use super::{
    GeometryCollectionTrait, LineStringTrait, MultiLineStringTrait, MultiPointTrait,
    MultiPolygonTrait, PointTrait, PolygonTrait, RectTrait,
};

#[allow(clippy::type_complexity)]
pub trait GeometryTrait<'a> {
    type T: CoordNum;
    type Point: 'a + PointTrait<T = Self::T>;
    type LineString: 'a + LineStringTrait<'a, T = Self::T>;
    type Polygon: 'a + PolygonTrait<'a, T = Self::T>;
    type MultiPoint: 'a + MultiPointTrait<'a, T = Self::T>;
    type MultiLineString: 'a + MultiLineStringTrait<'a, T = Self::T>;
    type MultiPolygon: 'a + MultiPolygonTrait<'a, T = Self::T>;
    type GeometryCollection: 'a + GeometryCollectionTrait<'a, T = Self::T>;
    type Rect: 'a + RectTrait<'a, T = Self::T>;

    fn as_type(
        &'a self,
    ) -> GeometryType<
        'a,
        Self::Point,
        Self::LineString,
        Self::Polygon,
        Self::MultiPoint,
        Self::MultiLineString,
        Self::MultiPolygon,
        Self::GeometryCollection,
        Self::Rect,
    >;

    fn into_type(self) -> GeometryType<
        'a,
        Self::Point,
        Self::LineString,
        Self::Polygon,
        Self::MultiPoint,
        Self::MultiLineString,
        Self::MultiPolygon,
        Self::GeometryCollection,
        Self::Rect,
    >;
}

#[derive(Debug)]
pub enum GeometryType<'a, P, L, Y, MP, ML, MY, GC, R>
where
    P: PointTrait,
    L: LineStringTrait<'a>,
    Y: PolygonTrait<'a>,
    MP: MultiPointTrait<'a>,
    ML: MultiLineStringTrait<'a>,
    MY: MultiPolygonTrait<'a>,
    GC: GeometryCollectionTrait<'a>,
    R: RectTrait<'a>,
{
    Point(&'a P),
    LineString(&'a L),
    Polygon(&'a Y),
    MultiPoint(&'a MP),
    MultiLineString(&'a ML),
    MultiPolygon(&'a MY),
    GeometryCollection(&'a GC),
    Rect(&'a R),
}

// #[derive(Debug)]
// pub enum OwnedGeometryType<'a, P, L, Y, MP, ML, MY, GC, R>
// where
//     P: PointTrait,
//     L: LineStringTrait<'a>,
//     Y: PolygonTrait<'a>,
//     MP: MultiPointTrait<'a>,
//     ML: MultiLineStringTrait<'a>,
//     MY: MultiPolygonTrait<'a>,
//     GC: GeometryCollectionTrait<'a>,
//     R: RectTrait<'a>,
// {
//     Point(P),
//     LineString(L),
//     Polygon(Y),
//     MultiPoint(MP),
//     MultiLineString(ML),
//     MultiPolygon(MY),
//     GeometryCollection(GC),
//     Rect(R),
// }

impl<'a, T: CoordNum + 'a> GeometryTrait<'a> for Geometry<T> {
    type T = T;
    type Point = Point<Self::T>;
    type LineString = LineString<Self::T>;
    type Polygon = Polygon<Self::T>;
    type MultiPoint = MultiPoint<Self::T>;
    type MultiLineString = MultiLineString<Self::T>;
    type MultiPolygon = MultiPolygon<Self::T>;
    type GeometryCollection = GeometryCollection<Self::T>;
    type Rect = Rect<Self::T>;

    fn as_type(
        &'a self,
    ) -> GeometryType<
        'a,
        Point<T>,
        LineString<T>,
        Polygon<T>,
        MultiPoint<T>,
        MultiLineString<T>,
        MultiPolygon<T>,
        GeometryCollection<T>,
        Rect<T>,
    > {
        match *self {
            Geometry::Point(ref p) => GeometryType::Point(p),
            Geometry::LineString(ref p) => GeometryType::LineString(p),
            Geometry::Polygon(ref p) => GeometryType::Polygon(p),
            Geometry::MultiPoint(ref p) => GeometryType::MultiPoint(p),
            Geometry::MultiLineString(ref p) => GeometryType::MultiLineString(p),
            Geometry::MultiPolygon(ref p) => GeometryType::MultiPolygon(p),
            Geometry::GeometryCollection(ref p) => GeometryType::GeometryCollection(p),
            Geometry::Rect(ref p) => GeometryType::Rect(p),
            _ => todo!(),
        }
    }

    fn into_type(
        self,
    ) -> GeometryType<
        'a,
        Point<T>,
        LineString<T>,
        Polygon<T>,
        MultiPoint<T>,
        MultiLineString<T>,
        MultiPolygon<T>,
        GeometryCollection<T>,
        Rect<T>,
    > {
        match self {
            Geometry::Point(ref p) => GeometryType::Point(p),
            Geometry::LineString(ref p) => GeometryType::LineString(p),
            Geometry::Polygon(ref p) => GeometryType::Polygon(p),
            Geometry::MultiPoint(ref p) => GeometryType::MultiPoint(p),
            Geometry::MultiLineString(ref p) => GeometryType::MultiLineString(p),
            Geometry::MultiPolygon(ref p) => GeometryType::MultiPolygon(p),
            Geometry::GeometryCollection(ref p) => GeometryType::GeometryCollection(p),
            Geometry::Rect(ref p) => GeometryType::Rect(p),
            _ => todo!(),
        }
    }


}
