use serde::{ser, Serialize};
use crate::quipu::{Quipu, Pendant};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Message(String),
    #[error("Unsupported type")]
    Unsupported,
}

impl ser::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

pub struct QuipuSerializer {
    pub output: Quipu,
}

pub struct PendantSerializer {
    pub output: Pendant,
}

pub fn to_quipu<T>(value: &T) -> Result<Quipu, Error>
where
    T: Serialize,
{
    let mut serializer = QuipuSerializer {
        output: Quipu::new(),
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

// ---------------- QuipuSerializer ----------------
// This serializer handles the root object.
// If it's a struct, its fields become Pendants.
// If it's a primitive, it becomes a single Pendant.

impl<'a> ser::Serializer for &'a mut QuipuSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<(), Error> { Err(Error::Unsupported) }

    // Primitives: Create a single pendant
    fn serialize_i8(self, v: i8) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_i16(self, v: i16) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_i32(self, v: i32) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_i64(self, v: i64) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u8(self, v: u8) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u16(self, v: u16) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u32(self, v: u32) -> Result<(), Error> { self.serialize_u64(v as u64) }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        let mut p = Pendant::new([255, 255, 255]);
        p.add_number(v);
        self.output.pendants.push(p);
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_f64(self, _v: f64) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_char(self, _v: char) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_str(self, _v: &str) -> Result<(), Error> { Err(Error::Unsupported) } // TODO: Implement string later
    fn serialize_bytes(self, _v: &[u8]) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_none(self) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error> where T: Serialize { value.serialize(self) }
    fn serialize_unit(self) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<(), Error> where T: Serialize { value.serialize(self) }
    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<(), Error> where T: Serialize { Err(Error::Unsupported) }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> { Err(Error::Unsupported) }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> { Err(Error::Unsupported) }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Error> { Err(Error::Unsupported) }
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Error> { Err(Error::Unsupported) }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> { Err(Error::Unsupported) }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Error> {
        Ok(self)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Error> { Err(Error::Unsupported) }
}

impl<'a> ser::SerializeStruct for &'a mut QuipuSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        // For each field of the root struct, we create a new Pendant.
        // We use PendantSerializer to populate it.
        let mut pendant_serializer = PendantSerializer {
            output: Pendant::new([255, 255, 255]),
        };
        value.serialize(&mut pendant_serializer)?;
        self.output.pendants.push(pendant_serializer.output);
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

// ---------------- PendantSerializer ----------------
// This serializer handles populating a Pendant.
// Primitives add knots.
// Struct fields add subsidiary Pendants.

impl<'a> ser::Serializer for &'a mut PendantSerializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = ser::Impossible<(), Error>;
    type SerializeTuple = ser::Impossible<(), Error>;
    type SerializeTupleStruct = ser::Impossible<(), Error>;
    type SerializeTupleVariant = ser::Impossible<(), Error>;
    type SerializeMap = ser::Impossible<(), Error>;
    type SerializeStruct = Self;
    type SerializeStructVariant = ser::Impossible<(), Error>;

    fn serialize_bool(self, _v: bool) -> Result<(), Error> { Err(Error::Unsupported) }

    fn serialize_i8(self, v: i8) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_i16(self, v: i16) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_i32(self, v: i32) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_i64(self, v: i64) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u8(self, v: u8) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u16(self, v: u16) -> Result<(), Error> { self.serialize_u64(v as u64) }
    fn serialize_u32(self, v: u32) -> Result<(), Error> { self.serialize_u64(v as u64) }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.output.add_number(v);
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_f64(self, _v: f64) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_char(self, _v: char) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_str(self, _v: &str) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_bytes(self, _v: &[u8]) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_none(self) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error> where T: Serialize { value.serialize(self) }
    fn serialize_unit(self) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_unit_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str) -> Result<(), Error> { Err(Error::Unsupported) }
    fn serialize_newtype_struct<T: ?Sized>(self, _name: &'static str, value: &T) -> Result<(), Error> where T: Serialize { value.serialize(self) }
    fn serialize_newtype_variant<T: ?Sized>(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _value: &T) -> Result<(), Error> where T: Serialize { Err(Error::Unsupported) }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> { Err(Error::Unsupported) }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> { Err(Error::Unsupported) }
    fn serialize_tuple_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeTupleStruct, Error> { Err(Error::Unsupported) }
    fn serialize_tuple_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeTupleVariant, Error> { Err(Error::Unsupported) }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> { Err(Error::Unsupported) }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct, Error> {
        Ok(self)
    }

    fn serialize_struct_variant(self, _name: &'static str, _variant_index: u32, _variant: &'static str, _len: usize) -> Result<Self::SerializeStructVariant, Error> { Err(Error::Unsupported) }
}

impl<'a> ser::SerializeStruct for &'a mut PendantSerializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        // For nested fields, we create subsidiary pendants.
        let mut sub = PendantSerializer {
            output: Pendant::new([200, 200, 200]),
        };
        value.serialize(&mut sub)?;
        self.output.subsidiaries.push(sub.output);
        Ok(())
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}
