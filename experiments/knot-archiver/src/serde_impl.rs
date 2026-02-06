use crate::quipu::{Cord, Knot};
use serde::{ser, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Custom error: {0}")]
    Custom(String),
    #[error("Unsupported type")]
    Unsupported,
    #[error("Serialization failed: {0}")]
    Message(String),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Message(msg.to_string())
    }
}

pub struct Serializer;

/// Helper to serialize any serializable value into a Quipu (wrapping the result Cord).
pub fn to_quipu<T>(value: &T) -> Result<crate::quipu::Quipu, Error>
where
    T: Serialize,
{
    let cord = value.serialize(Serializer)?;
    let mut q = crate::quipu::Quipu::new();
    q.add_pendant(cord);
    Ok(q)
}

impl ser::Serializer for Serializer {
    type Ok = Cord;
    type Error = Error;

    type SerializeSeq = Compound;
    type SerializeTuple = Compound;
    type SerializeTupleStruct = Compound;
    type SerializeTupleVariant = Compound;
    type SerializeMap = Compound;
    type SerializeStruct = Compound;
    type SerializeStructVariant = Compound;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        // boolean: 1 = true, 0 = false
        Ok(Cord::from_u64(if v { 1 } else { 0 }))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        // Quipus don't naturally do negative numbers, but we can treat them as unsigned
        // or maybe use a color/subsidiary to indicate sign?
        // For now, absolute value.
        Ok(Cord::from_u64(v.abs() as u64))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::from_u64(v))
    }

    fn serialize_f32(self, _v: f32) -> Result<Self::Ok, Self::Error> {
        // Floats not supported in this ancient system yet
        Err(Error::Unsupported)
    }

    fn serialize_f64(self, _v: f64) -> Result<Self::Ok, Self::Error> {
        Err(Error::Unsupported)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        self.serialize_u32(v as u32)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // String -> Sequence of chars
        // Root cord for string (value = length)
        let mut cord = Cord::from_u64(v.len() as u64);

        for c in v.chars() {
            let char_cord = Cord::from_u64(c as u64);
            cord.subsidiaries.push(char_cord);
        }
        Ok(cord)
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<Self::Ok, Self::Error> {
        let mut cord = Cord::from_u64(v.len() as u64);
        for &b in v {
            cord.subsidiaries.push(Cord::from_u64(b as u64));
        }
        Ok(cord)
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::default())
    }

    fn serialize_some<T: ?Sized>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::default())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::default())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::from_u64(variant_index as u64))
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: Serialize,
    {
        let mut cord = Cord::from_u64(variant_index as u64);
        let sub = value.serialize(self)?;
        cord.subsidiaries.push(sub);
        Ok(cord)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound {
            parent: Cord::default(),
        })
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound {
            parent: Cord::default(),
        })
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound {
            parent: Cord::default(),
        })
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Compound {
            parent: Cord::from_u64(variant_index as u64),
        })
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Compound {
            parent: Cord::default(),
        })
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound {
            parent: Cord::default(),
        })
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Compound {
            parent: Cord::from_u64(variant_index as u64),
        })
    }
}

pub struct Compound {
    parent: Cord,
}

impl ser::SerializeSeq for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}

impl ser::SerializeTuple for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_element<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}

impl ser::SerializeTupleStruct for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}

impl ser::SerializeTupleVariant for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}

impl ser::SerializeMap for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_key<T: ?Sized>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // Key is serialized as a cord, added.
        // Map: Key, Value, Key, Value...
        let sub = key.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn serialize_value<T: ?Sized>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}

impl ser::SerializeStruct for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        // We drop field names for now! Pure data.
        // (Or we could encode keys, but let's keep it simple "Archeologist" style: only the numbers matter)
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}

impl ser::SerializeStructVariant for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<(), Self::Error>
    where
        T: Serialize,
    {
        let sub = value.serialize(Serializer)?;
        self.parent.subsidiaries.push(sub);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.parent)
    }
}
