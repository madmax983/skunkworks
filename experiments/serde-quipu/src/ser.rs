use crate::error::Error;
use crate::quipu::{Cord, Quipu};
use serde::{ser, Serialize};

pub struct Serializer;

pub fn to_string<T: Serialize>(value: &T) -> Result<String, Error> {
    let cord = value.serialize(Serializer)?;
    let mut q = Quipu::new();
    q.add_cord(cord);
    Ok(q.to_string())
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
        Ok(Cord::from_u64(if v { 1 } else { 0 }, "Bool"))
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        self.serialize_i64(v as i64)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::from_u64(v.unsigned_abs(), "Int"))
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::from_u64(v, "UInt"))
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        self.serialize_u64(v as u64)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::from_u64(v as u64, "Char"))
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        // String is a sequence of chars (subsidiaries)
        let mut root = Cord::new("String");
        for c in v.chars() {
            let char_cord = Cord::from_u64(c as u64, "Char");
            root.add_subsidiary(char_cord);
        }
        Ok(root)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::new("Bytes"))
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::new("None"))
    }

    fn serialize_some<T>(self, value: &T) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::new("Unit"))
    }

    fn serialize_unit_struct(self, name: &'static str) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::new(name))
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        Ok(Cord::new(variant))
    }

    fn serialize_newtype_struct<T>(
        self,
        name: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut cord = value.serialize(self)?;
        cord.color = name.to_string();
        Ok(cord)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut cord = value.serialize(self)?;
        cord.color = variant.to_string();
        Ok(cord)
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        Ok(Compound::new("Seq"))
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        Ok(Compound::new("Tuple"))
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        Ok(Compound::new(name))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        Ok(Compound::new(variant))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        Ok(Compound::new("Map"))
    }

    fn serialize_struct(
        self,
        name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        Ok(Compound::new(name))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        Ok(Compound::new(variant))
    }
}

pub struct Compound {
    root: Cord,
}

impl Compound {
    fn new(name: &str) -> Self {
        Self {
            root: Cord::new(name),
        }
    }
}

impl ser::SerializeSeq for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(Serializer)?;
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

impl ser::SerializeTuple for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(Serializer)?;
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

impl ser::SerializeTupleStruct for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(Serializer)?;
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

impl ser::SerializeTupleVariant for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(Serializer)?;
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

impl ser::SerializeMap for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_key<T>(&mut self, key: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = key.serialize(Serializer)?;
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn serialize_value<T>(&mut self, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let cord = value.serialize(Serializer)?;
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

impl ser::SerializeStruct for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut cord = value.serialize(Serializer)?;
        cord.color = key.to_string();
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

impl ser::SerializeStructVariant for Compound {
    type Ok = Cord;
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<(), Self::Error>
    where
        T: ?Sized + Serialize,
    {
        let mut cord = value.serialize(Serializer)?;
        cord.color = key.to_string();
        self.root.add_subsidiary(cord);
        Ok(())
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        Ok(self.root)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Serialize)]
    struct User {
        id: u64,
        active: bool,
    }

    #[test]
    fn test_serialize_struct() {
        let u = User {
            id: 123,
            active: true,
        };
        let s = to_string(&u).unwrap();
        println!("{}", s);
        // User -> Root
        // +-- id (123) -> S-SS-L3
        // +-- active (1) -> E (or similar)

        assert!(s.contains("Q-ROOT"));
        assert!(s.contains("User"));
        assert!(s.contains("id"));
        assert!(s.contains("s-ss-L3")); // 123
        assert!(s.contains("active"));
    }
}
