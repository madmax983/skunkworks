use serde::{ser, Serialize};
use crate::quipu::{Cord, Quipu};
use thiserror::Error;
use std::fmt::Display;

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
    value.serialize(&mut serializer)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = Quipu;
    type Error = Error;

    type SerializeSeq = Compound;
    type SerializeTuple = Compound;
    type SerializeTupleStruct = Compound;
    type SerializeTupleVariant = Compound;
    type SerializeMap = Compound;
    type SerializeStruct = Compound;
    type SerializeStructVariant = Compound;

    fn serialize_bool(self, v: bool) -> Result<Quipu, Error> {
        let cords = if v {
            vec![Cord::from(1)] // 1 is true
        } else {
            vec![Cord::from(0)] // 0 is false
        };
        Ok(Quipu { cords })
    }

    fn serialize_i8(self, v: i8) -> Result<Quipu, Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Quipu, Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Quipu, Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Quipu, Error> {
        if v < 0 {
            return Err(Error::Unsupported("Quipu cannot represent negative numbers".into()));
        }
        self.serialize_u64(v as u64)
    }

    fn serialize_u8(self, v: u8) -> Result<Quipu, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Quipu, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Quipu, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Quipu, Error> {
        Ok(Quipu { cords: vec![Cord::from(v)] })
    }

    fn serialize_f32(self, _v: f32) -> Result<Quipu, Error> {
        Err(Error::Unsupported("Floats not supported".into()))
    }

    fn serialize_f64(self, _v: f64) -> Result<Quipu, Error> {
        Err(Error::Unsupported("Floats not supported".into()))
    }

    fn serialize_char(self, v: char) -> Result<Quipu, Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_str(self, v: &str) -> Result<Quipu, Error> {
        let mut cords = Vec::new();
        for c in v.chars() {
            cords.push(Cord::from(c as u64));
        }
        Ok(Quipu { cords })
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Quipu, Error> {
        let mut cords = Vec::new();
        for b in v {
            cords.push(Cord::from(*b as u64));
        }
        Ok(Quipu { cords })
    }

    fn serialize_none(self) -> Result<Quipu, Error> {
        Ok(Quipu { cords: vec![Cord::default()] }) // Empty cord for None
    }

    fn serialize_some<T>(self, value: &T) -> Result<Quipu, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Quipu, Error> {
        Ok(Quipu { cords: vec![] })
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Quipu, Error> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Quipu, Error> {
        self.serialize_unit()
    }

    fn serialize_newtype_struct<T>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Quipu, Error>
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
    ) -> Result<Quipu, Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Error> {
        Ok(Compound::new())
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Error> {
        Ok(Compound::new())
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Error> {
        Ok(Compound::new())
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Error> {
        Ok(Compound::new())
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Error> {
        Ok(Compound::new())
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Error> {
        Ok(Compound::new())
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Error> {
        Ok(Compound::new())
    }
}

pub struct Compound {
    quipu: Quipu,
}

impl Compound {
    fn new() -> Self {
        Compound {
            quipu: Quipu { cords: Vec::new() },
        }
    }
}

impl ser::SerializeSeq for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}

impl ser::SerializeTuple for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}

impl ser::SerializeTupleStruct for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}

impl ser::SerializeTupleVariant for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}

impl ser::SerializeMap for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        // We just append keys and values sequentially as cords
        let q = to_quipu(key)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}

impl ser::SerializeStruct for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}

impl ser::SerializeStructVariant for Compound {
    type Ok = Quipu;
    type Error = Error;

    fn serialize_field<T>(&mut self, _key: &'static str, value: &T) -> Result<(), Error>
    where
        T: ?Sized + Serialize,
    {
        let q = to_quipu(value)?;
        self.quipu.cords.extend(q.cords);
        Ok(())
    }

    fn end(self) -> Result<Quipu, Error> {
        Ok(self.quipu)
    }
}
