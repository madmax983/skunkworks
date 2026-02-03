use crate::quipu::Cord;
use serde::{Serialize, ser};
use std::fmt;

#[derive(Debug, Default)]
pub struct Khipu {
    pub cords: Vec<Cord>,
}

#[derive(Debug)]
pub struct Error(String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error(msg.to_string())
    }
}

pub struct Serializer {
    output: Khipu,
}

impl Serializer {
    pub fn new() -> Self {
        Serializer {
            output: Khipu::default(),
        }
    }
}

pub fn to_khipu<T>(value: &T) -> Result<Khipu, Error>
where
    T: Serialize,
{
    let mut serializer = Serializer::new();
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<(), Error> {
        self.output.cords.push(Cord::from(if v { 1 } else { 0 }));
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v.abs() as u32));
        Ok(())
    }

    fn serialize_i16(self, v: i16) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v.abs() as u32));
        Ok(())
    }

    fn serialize_i32(self, v: i32) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v.abs() as u32));
        Ok(())
    }

    fn serialize_i64(self, v: i64) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v.abs() as u32));
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v as u32));
        Ok(())
    }

    fn serialize_u16(self, v: u16) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v as u32));
        Ok(())
    }

    fn serialize_u32(self, v: u32) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v));
        Ok(())
    }

    fn serialize_u64(self, v: u64) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v as u32)); // Truncating for Cord (u32 internal mostly)
        Ok(())
    }

    fn serialize_f32(self, _v: f32) -> Result<(), Error> {
        Err(Error("Floats not supported in Ancient Quipu".to_string()))
    }

    fn serialize_f64(self, _v: f64) -> Result<(), Error> {
        Err(Error("Floats not supported in Ancient Quipu".to_string()))
    }

    fn serialize_char(self, v: char) -> Result<(), Error> {
        self.output.cords.push(Cord::from(v as u32));
        Ok(())
    }

    fn serialize_str(self, v: &str) -> Result<(), Error> {
        // Encode each char as a cord
        for c in v.chars() {
            self.output.cords.push(Cord::from(c as u32));
        }
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<(), Error> {
        for b in v {
            self.output.cords.push(Cord::from(*b as u32));
        }
        Ok(())
    }

    fn serialize_none(self) -> Result<(), Error> {
        // None could be an empty cord? Or skipped?
        // Let's make it a cord with 0 value.
        self.output.cords.push(Cord::default());
        Ok(())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Ok(self)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(self)
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Ok(self)
    }
}

// Compound implementations
// We just forward everything to the parent serializer because we are flattening everything into a list of cords.

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        // Ignore keys? Or map them to cords?
        // If we ignore keys, we just get values.
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        // Ignore field names
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: Serialize,
    {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<(), Error> {
        Ok(())
    }
}
