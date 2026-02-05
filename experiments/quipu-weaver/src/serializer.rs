use serde::{ser, Serialize};
use crate::quipu::{Quipu, Cord};
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Custom(String),
    UnsupportedType,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Custom(msg) => write!(f, "{}", msg),
            Error::UnsupportedType => write!(f, "Unsupported type"),
        }
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub struct Serializer {
    cord: Cord,
}

pub fn to_quipu<T>(value: &T) -> Result<Quipu, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer { cord: Cord::new() };
    value.serialize(&mut serializer)?;
    Ok(Quipu { primary_cord: serializer.cord })
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Compound<'a>;
    type SerializeTuple = Compound<'a>;
    type SerializeTupleStruct = Compound<'a>;
    type SerializeTupleVariant = Compound<'a>;
    type SerializeMap = Compound<'a>;
    type SerializeStruct = Compound<'a>;
    type SerializeStructVariant = Compound<'a>;

    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        self.cord = self.cord.clone().with_value(if v { 1 } else { 0 });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> { self.serialize_i64(v as i64) }
    fn serialize_i16(self, v: i16) -> Result<(), Error> { self.serialize_i64(v as i64) }
    fn serialize_i32(self, v: i32) -> Result<(), Error> { self.serialize_i64(v as i64) }
    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        self.cord = self.cord.clone().with_value(v.abs() as u64);
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u16(self, v: u16) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u32(self, v: u32) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.cord = self.cord.clone().with_value(v);
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<(), Error> { Err(Error::UnsupportedType) }
    fn serialize_f64(self, _v: f64) -> Result<(), Error> { Err(Error::UnsupportedType) }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
         // String encoded as bytes sequence on children cords
         for b in v.bytes() {
             let child = Cord::new().with_value(b as u64);
             self.cord.children.push(child);
         }
         Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Error> {
         for b in v {
             let child = Cord::new().with_value(*b as u64);
             self.cord.children.push(child);
         }
         Ok(())
    }

    fn serialize_none(self) -> Result<(), Error> { Ok(()) }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> { Ok(()) }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> { Ok(()) }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Error> { Ok(()) }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<(), Error> where T: Serialize {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(Compound { parent: &mut self.cord })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(Compound { parent: &mut self.cord })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Ok(Compound { parent: &mut self.cord })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Ok(Compound { parent: &mut self.cord })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(Compound { parent: &mut self.cord })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(Compound { parent: &mut self.cord })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Ok(Compound { parent: &mut self.cord })
    }
}

pub struct Compound<'a> {
    parent: &'a mut Cord,
}

impl<'a> ser::SerializeSeq for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let mut serializer = Serializer { cord: Cord::new() };
        value.serialize(&mut serializer)?;
        self.parent.children.push(serializer.cord);
        Ok(())
    }

    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'a> ser::SerializeTuple for Compound<'a> {
    type Ok = ();
    type Error = Error;
    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: Serialize {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'a> ser::SerializeTupleStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: Serialize {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'a> ser::SerializeTupleVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: Serialize {
        ser::SerializeSeq::serialize_element(self, value)
    }
    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'a> ser::SerializeMap for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Error> where T: Serialize {
         ser::SerializeSeq::serialize_element(self, key)
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error> where T: Serialize {
         ser::SerializeSeq::serialize_element(self, value)
    }

    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'a> ser::SerializeStruct for Compound<'a> {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        let mut serializer = Serializer { cord: Cord::new() };
        value.serialize(&mut serializer)?;

        let mut child_cord = serializer.cord;
        child_cord.label = Some(key.to_string());
        self.parent.children.push(child_cord);
        Ok(())
    }

    fn end(self) -> Result<(), Error> { Ok(()) }
}

impl<'a> ser::SerializeStructVariant for Compound<'a> {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: ?Sized>(&mut self, key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
         ser::SerializeStruct::serialize_field(self, key, value)
    }
    fn end(self) -> Result<(), Error> { Ok(()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::quipu::Knot;
    use serde::Serialize;

    #[test]
    fn test_simple_serialization() {
        #[derive(Serialize)]
        struct Point {
            x: i32,
            y: i32,
        }
        let p = Point { x: 10, y: 5 };
        let q = to_quipu(&p).unwrap();
        // Assert structure
        // x=10 -> 1 single knot at pos 1 (tens)
        // y=5 -> 1 long knot(5) at pos 0 (units)

        let x_cord = &q.primary_cord.children[0];
        let y_cord = &q.primary_cord.children[1];

        assert_eq!(x_cord.knots.len(), 1);
        // 10 is 1 Single knot at power 1.
        assert!(matches!(x_cord.knots[0].1, Knot::Single));

        assert_eq!(y_cord.knots.len(), 1);
        // 5 is Long(5) at power 0.
        if let Knot::Long(t) = &y_cord.knots[0].1 {
             assert_eq!(*t, 5);
        } else {
             panic!("Expected Long knot");
        }
    }
}
