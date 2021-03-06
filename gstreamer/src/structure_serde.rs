// Copyright (C) 2018 François Laignel <fengalin@free.fr>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use glib;
use glib::ToValue;

use serde::de;
use serde::de::{Deserialize, DeserializeSeed, Deserializer, SeqAccess, Visitor};
use serde::ser;
use serde::ser::{Serialize, SerializeSeq, SerializeTuple, Serializer};

use std::fmt;

use Buffer;
use DateTime;
use Sample;

use value::*;
use value_serde::*;

use Structure;
use StructureRef;

struct FieldSe<'a>(&'a str, &'a glib::SendValue);
impl<'a> Serialize for FieldSe<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        ser_value!(self.1, |type_, value| {
            let mut tup = serializer.serialize_tuple(3)?;
            tup.serialize_element(self.0)?;
            tup.serialize_element(type_)?;
            tup.serialize_element(&value)?;
            tup.end()
        })
    }
}

struct StructureForIter<'a>(&'a StructureRef);
impl<'a> Serialize for StructureForIter<'a> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let iter = self.0.iter();
        let size = iter.size_hint().0;
        if size > 0 {
            let mut seq = serializer.serialize_seq(Some(size))?;
            for field in iter {
                seq.serialize_element(&FieldSe(field.0, field.1))?;
            }
            seq.end()
        } else {
            let seq = serializer.serialize_seq(None)?;
            seq.end()
        }
    }
}

impl Serialize for StructureRef {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut tup = serializer.serialize_tuple(2)?;
        tup.serialize_element(self.get_name())?;
        tup.serialize_element(&StructureForIter(self))?;
        tup.end()
    }
}

impl Serialize for Structure {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

struct FieldDe(String, SendValue);
impl From<FieldDe> for (String, glib::SendValue) {
    fn from(field_de: FieldDe) -> Self {
        (field_de.0, field_de.1.into())
    }
}

struct FieldVisitor;
impl<'de> Visitor<'de> for FieldVisitor {
    type Value = FieldDe;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(
            "a tuple of 3 elements (name: `String`, type name: `String`, value: `Value`)",
        )
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value` name"))?;
        let type_name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value` type"))?;
        let send_value = de_send_value!(type_name, seq)?
            .ok_or_else(|| de::Error::custom("Expected a value for `Value`"))?;
        Ok(FieldDe(name, send_value))
    }
}

impl<'de> Deserialize<'de> for FieldDe {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple(3, FieldVisitor)
    }
}

struct FieldsDe<'a>(&'a mut StructureRef);

struct FieldsVisitor<'a>(&'a mut StructureRef);
impl<'de, 'a> Visitor<'de> for FieldsVisitor<'a> {
    type Value = ();

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a sequence of `Structure` `Field`s")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<(), A::Error> {
        while let Some(field) = seq.next_element::<FieldDe>()? {
            let (name, value): (String, glib::SendValue) = field.into();
            self.0.set_value(name.as_str(), value);
        }

        Ok(())
    }
}

impl<'de, 'a> DeserializeSeed<'de> for FieldsDe<'a> {
    type Value = ();

    fn deserialize<D: Deserializer<'de>>(self, deserializer: D) -> Result<(), D::Error> {
        deserializer.deserialize_seq(FieldsVisitor(self.0))
    }
}

struct StructureVisitor;
impl<'de> Visitor<'de> for StructureVisitor {
    type Value = Structure;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a `Structure` (name, sequence of `Field`s)")
    }

    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<Self::Value, A::Error> {
        let name = seq
            .next_element::<String>()?
            .ok_or_else(|| de::Error::custom("Expected a name for the `Structure`"))?;
        let mut structure = Structure::new_empty(&name);
        seq.next_element_seed(FieldsDe(structure.as_mut()))?
            .ok_or_else(|| de::Error::custom("Expected a sequence of `Field`s"))?;

        Ok(structure)
    }
}

impl<'de> Deserialize<'de> for Structure {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_tuple(2, StructureVisitor)
    }
}

#[cfg(test)]
mod tests {
    extern crate ron;

    use Array;
    use Fraction;
    use Structure;

    #[test]
    fn test_serialize() {
        ::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", &"abc")
            .field("f2", &String::from("bcd"))
            .field("f3", &123i32)
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .build();

        let mut pretty_config = ron::ser::PrettyConfig::default();
        pretty_config.new_line = "".to_string();

        let res = ron::ser::to_string_pretty(&s, pretty_config);
        assert_eq!(
            Ok(concat!(
                "(\"test\", [",
                "    (\"f1\", \"String\", \"abc\"),",
                "    (\"f2\", \"String\", \"bcd\"),",
                "    (\"f3\", \"i32\", 123),",
                "    (\"fraction\", \"Fraction\", (1, 2)),",
                "    (\"array\", \"Array\", [",
                "        (\"i32\", 1),",
                "        (\"i32\", 2),",
                "    ]),",
                "])"
            )
            .to_owned()),
            res,
        );
    }

    #[test]
    fn test_deserialize() {
        ::init().unwrap();

        let s_ron = r#"
            ("test", [
                ("f1", "String", "abc"),
                ("f2", "String", "bcd"),
                ("f3", "i32", 123),
                ("fraction", "Fraction", (1, 2)),
                ("array", "Array", [
                    ("i32", 1),
                    ("i32", 2),
                ]),
            ])"#;
        let s: Structure = ron::de::from_str(s_ron).unwrap();
        assert_eq!(
            s.as_ref(),
            Structure::new(
                "test",
                &[
                    ("f1", &"abc"),
                    ("f2", &"bcd"),
                    ("f3", &123),
                    ("fraction", &Fraction::new(1, 2)),
                    ("array", &Array::new(&[&1, &2])),
                ],
            )
            .as_ref()
        );
    }

    #[test]
    fn test_serde_roundtrip() {
        ::init().unwrap();

        let s = Structure::builder("test")
            .field("f1", &"abc")
            .field("f2", &String::from("bcd"))
            .field("f3", &123i32)
            .field("fraction", &Fraction::new(1, 2))
            .field("array", &Array::new(&[&1, &2]))
            .build();
        let s_ser = ron::ser::to_string(&s).unwrap();
        let s_de: Structure = ron::de::from_str(s_ser.as_str()).unwrap();
        assert_eq!(s_de.as_ref(), s.as_ref());
    }
}
