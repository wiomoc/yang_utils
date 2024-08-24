use crate::errors::ErrorContext;
use crate::parser::model;
use crate::typing::range::IntRangePart;
use phf::phf_map;
use std::slice::Iter;
use crate::typing::number::{DecimalType, IntType};
use crate::typing::string::StringType;
use crate::typing::enumeration::EnumerationType;


mod number;
mod range;
mod string;
mod boolean;
mod enumeration;

#[derive(Debug, Clone)]
pub(crate) enum SingleItemOrVec<T> {
    Single(T),
    Vec(Vec<T>),
}

impl<T> SingleItemOrVec<T> {
    pub(crate) fn iter(&self) -> SingleItemOrVecIter<T> {
        match self {
            SingleItemOrVec::Single(value) => SingleItemOrVecIter::Single(Some(value)),
            SingleItemOrVec::Vec(vec) => SingleItemOrVecIter::Vec(vec.iter()),
        }
    }
}

pub(crate) enum SingleItemOrVecIter<'a, T> {
    Single(Option<&'a T>),
    Vec(Iter<'a, T>),
}

impl<'a, T> Iterator for SingleItemOrVecIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SingleItemOrVecIter::Single(value) => value.take(),
            SingleItemOrVecIter::Vec(iter) => iter.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            SingleItemOrVecIter::Single(value) => {
                if value.is_some() {
                    (1, Some(1))
                } else {
                    (0, Some(0))
                }
            }
            SingleItemOrVecIter::Vec(iter) => iter.size_hint(),
        }
    }
}

trait TypeTrait: Sized {
    fn get_rust_type(&self) -> &'static str;
    fn derive(&self, model: &model::Type, error_context: &mut ErrorContext) -> Result<Self, ()>;
    fn is_abstract_type(&self) -> bool {
        false
    }
}

macro_rules! collect_unused_fields {
    ($model:ident, [$($field:ident),*]) => {
        {
            let mut unused_fields = vec![];
            $(
                if !$model.$field.iter().next().is_some() {
                    unused_fields.push(stringify!($field));
                }
            )*
            unused_fields
        }
    };
}

pub(crate) use collect_unused_fields;
use crate::typing::boolean::BoolType;

enum Type {
    Int(IntType),
    Decimal(DecimalType),
    String(StringType),
    Bool(BoolType),
    Enumeration(EnumerationType),
}

static BUILD_IN_TYPES: phf::Map<&'static str, Type> = phf_map! {
    "uint8" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: 0, max_value: 8 }),
        rust_type: "u8"
    }),
     "uint16" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: 0, max_value: 16 }),
        rust_type: "u16"
    }),
     "uint32" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: 0, max_value: 32 }),
        rust_type: "u32"
    }),
     "uint64" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: 0, max_value: 64 }),
        rust_type: "u64"
    }),
     "int8" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: -128, max_value: 127 }),
        rust_type: "i8"
    }),
     "int16" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: -32768, max_value: 32767 }),
        rust_type: "i16"
    }),
     "int32" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: -2147483648, max_value: 2147483647 }),
        rust_type: "i32"
    }),
     "int64" => Type::Int(IntType {
        range: SingleItemOrVec::Single(IntRangePart { min_value: -9223372036854775808, max_value: 9223372036854775807 }),
        rust_type: "i64"
    }),
    "decimal64" => Type::Decimal(DecimalType {
        range: None,
        fraction_digits: None
    }),
    "string" => Type::String(StringType {
        length: SingleItemOrVec::Single(IntRangePart { min_value: 0, max_value: usize::MAX as i128 }),
        pattern: None
    }),
    "boolean" => Type::Bool(BoolType {}),
    "enumeration" => Type::Enumeration(EnumerationType { items: None }),
};
