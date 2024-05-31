use serde::{ser, Serialize};

use crate::error::{Error, Result};

// By convention, the public API of a Serde serializer is one or more `to_abc`
// functions such as `to_string`, `to_bytes`, or `to_writer` depending on what
// Rust types the serializer is able to produce as output.
//
// This basic serializer supports only `to_string`.
pub fn to_value<T>(value: &T) -> Result<rmpv::Value>
where
    T: Serialize,
{
    let mut serializer = Serializer {
        output: rmpv::Value::Nil,
    };
    value.serialize(&mut serializer)?;
    Ok(serializer.output)
}

pub struct Serializer {
    output: rmpv::Value,
}

impl Serializer {
    // Serialize a single element of the sequence.
    fn serialize_seq_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            rmpv::Value::Array(ref mut vec) => {
                let mut serializer = Serializer {
                    output: rmpv::Value::Nil,
                };
                value.serialize(&mut serializer)?;
                vec.push(serializer.output);
                Ok(())
            }
            _ => Err(Error::Message("expected array".to_string())),
        }
    }
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();

    // The error type when some error occurs during serialization.
    type Error = Error;

    // Associated types for keeping track of additional state while serializing
    // compound data structures like sequences and maps. In this case no
    // additional state is required beyond what is already stored in the
    // Serializer struct.
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.output = rmpv::Value::Boolean(v);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    // Not particularly efficient but this is example code anyway. A more
    // performant approach would be to use the `itoa` crate.
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.output = rmpv::Value::Integer(v.into());
        Ok(())
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        self.output = rmpv::Value::Integer(v.into());
        Ok(())
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        self.output = rmpv::Value::F64(v);
        Ok(())
    }

    // Serialize a char as a single-character string.
    fn serialize_char(self, v: char) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        self.output = rmpv::Value::String(v.to_string().into());
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        self.output = rmpv::Value::Binary(v.into());
        Ok(())
    }

    // A present optional is represented as just the contained value.
    fn serialize_some<T>(self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    // In Serde, unit means an anonymous value containing no data. Map this to
    // msgpack as `null`.
    fn serialize_unit(self) -> Result<()> {
        self.output = rmpv::Value::Nil;
        Ok(())
    }

    // Unit struct means a named value containing no data. Again, since there is
    // no data, map this to msgpack as `nil`.
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    // When serializing a unit variant (or any other kind of variant), formats
    // can choose whether to keep track of it by index or by name. Binary
    // formats typically use the index of the variant and human-readable formats
    // typically use the name.
    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain.
    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        value.serialize(self)
    }

    fn serialize_none(self) -> Result<()> {
        self.serialize_unit()
    }

    // NewType variants are represented as Array<Vec[ENUM_NAME, VARIANT_NAME, DATA]>
    fn serialize_newtype_variant<T>(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        let mut serializer = Serializer {
            output: rmpv::Value::Nil,
        };
        value.serialize(&mut serializer)?;

        self.output = rmpv::Value::Array(vec![
            rmpv::Value::String(name.into()),
            rmpv::Value::String(variant.into()),
            serializer.output,
        ]);
        Ok(())
    }

    // Now we get to the serialization of compound types.
    //
    // The start of the sequence, each value, and the end are three separate
    // method calls.
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.output = rmpv::Value::Array(Vec::new());
        Ok(self)
    }

    // Tuples look just like sequences.
    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_seq(Some(len))
    }

    // Tuple structs look just like sequences.
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_seq(Some(len))
    }

    // Tuple variants are represented as Array<Vec[ENUM_NAME, VARIANT_NAME, ... DATA ...]>.
    fn serialize_tuple_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        self.output = rmpv::Value::Array(vec![
            rmpv::Value::String(name.into()),
            rmpv::Value::String(variant.into()),
        ]);
        Ok(self)
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        self.output = rmpv::Value::Map(Vec::new());
        Ok(self)
    }

    // Structs look just like maps.
    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.serialize_map(Some(len))
    }

    // Struct variants are represented as `[ ENUM_NAME, VARIANT_NAME: { K: V, ... } ]`.
    // This is the externally tagged representation.
    fn serialize_struct_variant(
        self,
        name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        self.output = rmpv::Value::Array(vec![
            rmpv::Value::String(name.into()),
            rmpv::Value::String(variant.into()),
            rmpv::Value::Map(Vec::new()),
        ]);
        Ok(self)
    }
}

// This impl is SerializeSeq so these methods are called after `serialize_seq`
// is called on the Serializer.
impl<'a> ser::SerializeSeq for &'a mut Serializer {
    // Must match the `Ok` type of the serializer.
    type Ok = ();
    // Must match the `Error` type of the serializer.
    type Error = Error;

    // Serialize a single element of the sequence.
    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_seq_element(value)
    }

    // Close the sequence. Does nothing in this case.
    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuples.
impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_seq_element(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Same thing but for tuple structs.
impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_seq_element(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

/// Tuple variants are represented as Array<Vec[NAME, ... DATA ...]>.
impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        self.serialize_seq_element(value)
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Some `Serialize` types are not able to hold a key and value in memory at the
// same time so `SerializeMap` implementations are required to support
// `serialize_key` and `serialize_value` individually.
//
// There is a third optional method on the `SerializeMap` trait. The
// `serialize_entry` method allows serializers to optimize for the case where
// key and value are both available simultaneously. In JSON it doesn't make a
// difference so the default behavior for `serialize_entry` is fine.
impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    // The Serde data model allows map keys to be any serializable type. JSON
    // only allows string keys so the implementation below will produce invalid
    // JSON if the key serializes as something other than a string.
    //
    // A real JSON serializer would need to validate that map keys are strings.
    // This can be done by using a different Serializer to serialize the key
    // (instead of `&mut **self`) and having that other serializer only
    // implement `serialize_str` and return an error on any other data type.
    fn serialize_key<T>(&mut self, key: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            rmpv::Value::Map(ref mut vec) => {
                let mut serializer = Serializer {
                    output: rmpv::Value::Nil,
                };
                key.serialize(&mut serializer)?;
                vec.push((serializer.output, rmpv::Value::Nil));
                Ok(())
            }
            _ => Err(Error::Message("expected map".to_string())),
        }
    }

    // It doesn't make a difference whether the colon is printed at the end of
    // `serialize_key` or at the beginning of `serialize_value`. In this case
    // the code is a bit simpler having it here.
    fn serialize_value<T>(&mut self, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            rmpv::Value::Map(ref mut vec) => {
                let mut serializer = Serializer {
                    output: rmpv::Value::Nil,
                };
                value.serialize(&mut serializer)?;
                let last = vec.len() - 1;
                vec[last].1 = serializer.output;
                Ok(())
            }
            _ => Err(Error::Message("expected map".to_string())),
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Structs are like maps in which the keys are constrained to be compile-time
// constant strings.
impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            rmpv::Value::Map(ref mut vec) => {
                let mut keyser = Serializer {
                    output: rmpv::Value::Nil,
                };
                key.serialize(&mut keyser)?;

                let mut valser = Serializer {
                    output: rmpv::Value::Nil,
                };
                value.serialize(&mut valser)?;

                vec.push((keyser.output, valser.output));
                Ok(())
            }
            _ => Err(Error::Message("expected map".to_string())),
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

// Similar to `SerializeTupleVariant`, here the `end` method is responsible for
// closing both of the curly braces opened by `serialize_struct_variant`.
impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T>(&mut self, key: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        match &mut self.output {
            rmpv::Value::Array(ref mut vec) => {
                let mut serializer = Serializer {
                    output: rmpv::Value::Nil,
                };
                value.serialize(&mut serializer)?;

                let last_off = vec.len() - 1;
                let last = &mut vec[last_off];
                match last {
                    rmpv::Value::Map(ref mut map) => {
                        map.push((rmpv::Value::String(key.into()), serializer.output));
                    }
                    _ => return Err(Error::Message("expected map".to_string())),
                }
                Ok(())
            }
            _ => Err(Error::Message("expected array".to_string())),
        }
    }

    fn end(self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;

    use serde_derive::Serialize;

    #[test]
    fn test_serialize() {
        let v: u64 = 23;
        assert_eq!(to_value(&v).unwrap(), rmpv::Value::from(23));
        assert_eq!(
            to_value(&[1, 2, 3]).unwrap(),
            rmpv::Value::Array(vec![
                rmpv::Value::from(1),
                rmpv::Value::from(2),
                rmpv::Value::from(3)
            ])
        );

        #[derive(Serialize)]
        struct TupleStruct(u8, u8);
        assert_eq!(
            to_value(&TupleStruct(1, 2)).unwrap(),
            rmpv::Value::Array(vec![rmpv::Value::from(1), rmpv::Value::from(2)])
        );

        #[derive(Serialize)]
        enum TEnum {
            Tuple(u8, u8),
            Newtype(u8),
            Struct { a: u8, b: u8 },
        }
        assert_eq!(
            to_value(&TEnum::Tuple(1, 2)).unwrap(),
            rmpv::Value::Array(vec![
                rmpv::Value::String("TEnum".into()),
                rmpv::Value::String("Tuple".into()),
                rmpv::Value::from(1),
                rmpv::Value::from(2)
            ])
        );
        assert_eq!(
            to_value(&TEnum::Newtype(2)).unwrap(),
            rmpv::Value::Array(vec![
                rmpv::Value::String("TEnum".into()),
                rmpv::Value::String("Newtype".into()),
                rmpv::Value::from(2)
            ])
        );
        assert_eq!(
            to_value(&TEnum::Struct { a: 1, b: 2 }).unwrap(),
            rmpv::Value::Array(vec![
                rmpv::Value::String("TEnum".into()),
                rmpv::Value::String("Struct".into()),
                rmpv::Value::Map(vec![
                    (rmpv::Value::String("a".into()), rmpv::Value::from(1)),
                    (rmpv::Value::String("b".into()), rmpv::Value::from(2))
                ])
            ])
        );

        let map = {
            let mut map = HashMap::new();
            map.insert("a", 1);
            map
        };
        assert_eq!(
            to_value(&map).unwrap(),
            rmpv::Value::Map(vec![(
                rmpv::Value::String("a".into()),
                rmpv::Value::from(1)
            ),])
        );

        #[derive(Serialize)]
        struct S {
            a: u8,
            b: u8,
        }
        assert_eq!(
            to_value(&S { a: 1, b: 2 }).unwrap(),
            rmpv::Value::Map(vec![
                (rmpv::Value::String("a".into()), rmpv::Value::from(1)),
                (rmpv::Value::String("b".into()), rmpv::Value::from(2))
            ])
        );
    }
}
