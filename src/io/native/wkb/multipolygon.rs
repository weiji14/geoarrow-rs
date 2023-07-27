use std::io::Cursor;
use std::iter::Cloned;
use std::slice::Iter;

use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

use crate::geo_traits::MultiPolygonTrait;
use crate::io::native::wkb::geometry::Endianness;
use crate::io::native::wkb::polygon::WKBPolygon;

const HEADER_BYTES: u64 = 5;

#[derive(Debug, Clone)]
pub struct WKBMultiPolygon<'a> {
    // buf: &'a [u8],
    // byte_order: Endianness,

    // /// The number of polygons in this MultiPolygon
    // num_polygons: usize,

    // /// The offset in the buffer where each WKBPolygon object begins
    // ///
    // /// The length of this vec must match the number of polygons
    // // polygon_offsets: Vec<usize>,
    /// A WKBPolygon object for each of the internal line strings
    wkb_polygons: Vec<WKBPolygon<'a>>,
}

impl<'a> WKBMultiPolygon<'a> {
    pub fn new(buf: &'a [u8], byte_order: Endianness) -> Self {
        let mut reader = Cursor::new(buf);
        reader.set_position(HEADER_BYTES);
        let num_polygons = match byte_order {
            Endianness::BigEndian => reader.read_u32::<BigEndian>().unwrap().try_into().unwrap(),
            Endianness::LittleEndian => reader
                .read_u32::<LittleEndian>()
                .unwrap()
                .try_into()
                .unwrap(),
        };

        // - 1: byteOrder
        // - 4: wkbType
        // - 4: numLineStrings
        let mut polygon_offset = 1 + 4 + 4;
        let mut wkb_polygons = Vec::with_capacity(num_polygons);
        for _ in 0..num_polygons {
            let polygon = WKBPolygon::new(buf, byte_order, polygon_offset);
            polygon_offset += polygon.size();
            wkb_polygons.push(polygon);
        }

        Self { wkb_polygons }
    }
}

impl<'a> MultiPolygonTrait<'a> for WKBMultiPolygon<'a> {
    type T = f64;
    type ItemType = WKBPolygon<'a>;
    type Iter = Cloned<Iter<'a, Self::ItemType>>;

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    fn polygon(&self, i: usize) -> Option<Self::ItemType> {
        if i > self.num_polygons() {
            return None;
        }

        Some(self.wkb_polygons[i].clone())
    }

    fn polygons(&'a self) -> Self::Iter {
        todo!()
    }
}

impl<'a> MultiPolygonTrait<'a> for &WKBMultiPolygon<'a> {
    type T = f64;
    type ItemType = WKBPolygon<'a>;
    type Iter = Cloned<Iter<'a, Self::ItemType>>;

    fn num_polygons(&self) -> usize {
        self.wkb_polygons.len()
    }

    fn polygon(&self, i: usize) -> Option<Self::ItemType> {
        if i > self.num_polygons() {
            return None;
        }

        Some(self.wkb_polygons[i].clone())
    }

    fn polygons(&'a self) -> Self::Iter {
        todo!()
    }
}