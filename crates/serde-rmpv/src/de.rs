use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Deserialize;

use crate::error::*;

pub struct Deserializer<'de> {
    input: &'de rmpv::Value,
}

impl<'de> Deserializer<'de> {
    pub fn from_value(input: &'de rmpv::Value) -> Self {
        Deserializer { input }
    }
}

pub fn from_value<'a, T>(s: &'a rmpv::Value) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_value(s);
    T::deserialize(&mut deserializer)
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    // Look at the input data to decide what Serde data model type to
    // deserialize as. Not all data formats are able to support this operation.
    // Formats that support `deserialize_any` are known as self-describing.
    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            rmpv::Value::Boolean(_) => self.deserialize_bool(visitor),
            _ => Err(Error::UnsupportedType),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_bool(
            self.input
                .as_bool()
                .ok_or(Error::TypeError("expected bool".to_string()))?,
        )
    }

    // The `parse_signed` function is generic over the integer type `T` so here
    // it is invoked with `T=i8`. The next 8 methods are similar.
    fn deserialize_i8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_i16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_i32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_i64(self.input.as_i64().expect("i64"))
    }

    fn deserialize_u8<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_u16<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_u32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_u64(self.input.as_u64().expect("u64"))
    }

    fn deserialize_f32<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_f64(self.input.as_f64().expect("f64"))
    }

    fn deserialize_char<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    // Refer to the "Understanding deserializer lifetimes" page for information
    // about the three deserialization flavors of strings in Serde.
    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.input.as_str().expect("string"))
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_borrowed_str(self.input.as_str().expect("string"))
    }

    fn deserialize_bytes<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_byte_buf<V>(self, _visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        unimplemented!()
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            rmpv::Value::Nil => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            rmpv::Value::Nil => visitor.visit_unit(),
            _ => Err(Error::TypeError("expected nil".to_string())),
        }
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        Err(Error::UnsupportedType)
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.input {
            rmpv::Value::Array(_) => visitor.visit_seq(ArrayAccess::new(self)),
            _ => Err(Error::TypeError("expected array".to_string())),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if let rmpv::Value::Map(_) = self.input {
            visitor.visit_map(ValueMapAccess::new(self))
        } else {
            Err(Error::TypeError("expected map".to_string()))
        }
    }

    // The type that identifies a field of a struct or the variant of an enum.
    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_any(visitor)
    }
}

struct ArrayAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    offset: usize,
}

impl<'a, 'de> ArrayAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        ArrayAccess { de, offset: 0 }
    }
}

impl<'de, 'a> SeqAccess<'de> for ArrayAccess<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        let arr = self.de.input.as_array().expect("array");
        if self.offset < arr.len() {
            let value = arr[self.offset].clone();
            self.offset += 1;
            Ok(Some(
                seed.deserialize(value)
                    .map_err(|e| Error::Message(e.to_string()))?,
            ))
        } else {
            Ok(None)
        }
    }
}

struct ValueMapAccess<'a, 'de: 'a> {
    de: &'a mut Deserializer<'de>,
    offset: usize,
}

impl<'a, 'de> ValueMapAccess<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        ValueMapAccess { de, offset: 0 }
    }
}

impl<'de, 'a> MapAccess<'de> for ValueMapAccess<'a, 'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        let m = self.de.input.as_map().expect("map");
        if self.offset < m.len() {
            let key = m[self.offset].0.clone();
            self.offset += 1;
            Ok(Some(
                seed.deserialize(key)
                    .map_err(|e| Error::Message(e.to_string()))?,
            ))
        } else {
            Ok(None)
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let m = self.de.input.as_map().expect("map");
        let value = m[self.offset - 1].1.clone();
        seed.deserialize(value)
            .map_err(|e| Error::Message(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_enum() {
        use super::*;

        let v = rmpv::Value::from("string");
        assert_eq!("string", from_value::<String>(&v).unwrap());

        from_value::<i8>(&v).expect_err("expected unimplemented");
    }
}
