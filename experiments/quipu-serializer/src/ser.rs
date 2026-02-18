use quipu::{Color, Cord, Quipu};
use serde::{ser, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Custom: {0}")]
    Custom(String),
    #[error("Unsupported type: {0}")]
    Unsupported(String),
}

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub struct Serializer;

pub fn to_quipu<T>(value: &T) -> Result<Quipu, Error>
where
    T: Serialize + ?Sized,
{
    let mut serializer = Serializer;
    let cord = value.serialize(&mut serializer)?;
    // The result is a single cord (potentially with subsidiaries).
    // In a Quipu, this acts as one pendant cord.
    Ok(Quipu { cords: vec![cord] })
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Cord;
    type Error = Error;

    type SerializeSeq = Compound;
    type SerializeTuple = Compound;
    type SerializeTupleStruct = Compound;
    type SerializeTupleVariant = Compound;
    type SerializeMap = Compound;
    type SerializeStruct = Compound;
    type SerializeStructVariant = Compound;

    fn serialize_bool(self, v: bool) -> Result<Cord, Error> {
        let mut cord = if v { Cord::from(1) } else { Cord::from(0) };
        cord.color = Color::Blue; // Bool: Blue
        Ok(cord)
    }

    fn serialize_i8(self, v: i8) -> Result<Cord, Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Cord, Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Cord, Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Cord, Error> {
        if v < 0 {
            return Err(Error::Unsupported(
                "Quipu cannot represent negative numbers".into(),
            ));
        }
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, v: u8) -> Result<Cord, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Cord, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Cord, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Cord, Error> {
        let mut cord = Cord::from(v);
        cord.color = Color::Natural; // Number: Natural
        Ok(cord)
    }

    fn serialize_f32(self, _v: f32) -> Result<Cord, Error> {
        Err(Error::Unsupported("Floats not supported".into()))
    }

    fn serialize_f64(self, _v: f64) -> Result<Cord, Error> {
        Err(Error::Unsupported("Floats not supported".into()))
    }

    fn serialize_char(self, v: char) -> Result<Cord, Error> {
        let mut cord = Cord::from(v as u64);
        cord.color = Color::Green; // Char: Green
        Ok(cord)
    }

    fn serialize_str(self, v: &str) -> Result<Cord, Error> {
        // String represented as a cord with length value, and chars as subsidiaries
        let mut cord = Cord::from(v.len() as u64);
        cord.color = Color::Green; // String: Green

        for c in v.chars() {
            let mut char_cord = Cord::from(c as u64);
            char_cord.color = Color::Natural;
            cord.subsidiaries.push(char_cord);
        }
        Ok(cord)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Cord, Error> {
        let mut cord = Cord::from(v.len() as u64);
        cord.color = Color::Green;
        for b in v {
            let mut byte_cord = Cord::from(*b as u64);
            byte_cord.color = Color::Natural;
            cord.subsidiaries.push(byte_cord);
        }
        Ok(cord)
    }

    fn serialize_none(self) -> Result<Cord, Error> {
        let mut cord = Cord::default();
        cord.color = Color::Black; // None: Black
        Ok(cord)
    }

    fn serialize_some<T>(self, value: &T) -> Result<Cord, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Cord, Error> {
        let mut cord = Cord::default();
        cord.color = Color::White; // Unit: White
        Ok(cord)
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Cord, Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Cord, Error> {
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<Cord, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Cord, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(Compound::new(len.unwrap_or(0), Color::Yellow)) // Seq: Yellow
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(Compound::new(len, Color::Yellow))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Ok(Compound::new(len, Color::Red)) // TupleStruct: Red
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Ok(Compound::new(len, Color::Red))
    }

    fn serialize_map(self, len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(Compound::new(len.unwrap_or(0), Color::Red)) // Map: Red
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(Compound::new(len, Color::Red)) // Struct: Red
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Ok(Compound::new(len, Color::Red))
    }
}

pub struct Compound {
    subsidiaries: Vec<Cord>,
    base_color: Color,
    len_hint: usize,
}

impl Compound {
    fn new(len_hint: usize, color: Color) -> Self {
        Compound {
            subsidiaries: Vec::with_capacity(len_hint),
            base_color: color,
            len_hint,
        }
    }
}

impl ser::SerializeSeq for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        // Recursively serialize element into a Cord
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        // Create the container cord
        // Value = length of subsidiaries
        let mut cord = Cord::from(self.subsidiaries.len() as u64);
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}

impl ser::SerializeTuple for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        let mut cord = Cord::from(self.subsidiaries.len() as u64);
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}

impl ser::SerializeTupleStruct for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        let mut cord = Cord::from(self.subsidiaries.len() as u64);
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}

impl ser::SerializeTupleVariant for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        let mut cord = Cord::from(self.subsidiaries.len() as u64);
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}

impl ser::SerializeMap for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = key.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        let mut cord = Cord::from(self.subsidiaries.len() as u64); // Count keys + values
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}

impl ser::SerializeStruct for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        // Note: Field names are lost in Quipu (unless we serialize them as keys?)
        // Standard serialize_struct usually ignores keys for output formats that are positional.
        // If we want keys, we'd need to serialize _key as a string cord and push it.
        // Let's keep it positional for now, or maybe add field name if we want.
        // To keep it simple: positional.
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        let mut cord = Cord::from(self.subsidiaries.len() as u64);
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}

impl ser::SerializeStructVariant for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(&mut Serializer)?;
        self.subsidiaries.push(cord);
        Ok(())
    }

    fn end(self) -> Result<Cord, Error> {
        let mut cord = Cord::from(self.subsidiaries.len() as u64);
        cord.color = self.base_color;
        cord.subsidiaries = self.subsidiaries;
        Ok(cord)
    }
}
