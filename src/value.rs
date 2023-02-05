//! Attribute values and handling different types that can be converted to a value
use std::fmt;

/// Wrapper type for attribute values to allow setting attributes to floats or ints easily
#[derive(Debug, Clone)]
pub struct Value {
    value: String,
}

impl Value {
    pub fn to_string_bare(&self) -> String {
        self.value.clone()
    }
}

impl fmt::Display for Value {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: handle character escaping
        if self.value.contains('\"') {
            write!(formatter, "'{}'", self.value)
        } else {
            write!(formatter, "\"{}\"", self.value)
        }
    }
}

impl From<String> for Value {
    fn from(other: String) -> Value {
        Value { value: other }
    }
}

macro_rules! implement_from_to_string {
    ($($primitive:ty,)*) => (
        $(impl From<$primitive> for Value {
            #[inline]
            fn from(inner: $primitive) -> Self {
                Value {
                    value: inner.to_string()
                }
            }
        })*
    );
}

implement_from_to_string! {
    &str,
    i8, i16, i32, i64, isize,
    u8, u16, u32, u64, usize,
    f32, f64,
    bool,
}
