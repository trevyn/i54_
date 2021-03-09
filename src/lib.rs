#![forbid(unsafe_code)]

#[cfg(doctest)]
doc_comment::doctest!("../README.md");

use serde::{Deserialize, Serialize};
use std::{
    convert::{TryFrom, TryInto},
    ops::Add,
    ops::AddAssign,
};
use ux_serde::i54 as ux_i54;

#[derive(thiserror::Error, Debug)]
#[allow(non_camel_case_types)]
pub enum i54Error {
    #[error("i54 conversion failed")]
    ConversionFailed,
}

pub const MAX_SAFE_INTEGER: i64 = 9007199254740991;
pub const MIN_SAFE_INTEGER: i64 = -9007199254740991;

impl std::str::FromStr for i54 {
    type Err = String;
    fn from_str(_value: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

// And we define how to represent i54 as a string.
impl std::fmt::Display for i54 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize, Serialize, Copy, Clone, Debug)]
#[allow(non_camel_case_types)]
pub struct i54(ux_i54);

impl<T> PartialEq<T> for i54
where
    T: TryInto<i54> + Copy + PartialEq + Ord,
{
    fn eq(&self, other: &T) -> bool {
        match (*other).try_into() {
            Ok(v) => v.0 == self.0,
            Err(_) => false,
        }
    }
}

impl PartialEq for i54 {
    fn eq(&self, other: &i54) -> bool {
        self.0 == other.0
    }
}

impl Eq for i54 {}

#[cfg(feature = "juniper")]
#[juniper::graphql_scalar(
    description = "i54: 54-bit signed integer abstraction; represented as `i54`/`i64` in Rust, `Float` in GraphQL, `number` in TypeScript."
)]
impl<S> GraphQLScalar for i54
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        let val: i64 = self.0.into();
        juniper::Value::scalar(val as f64)
    }

    // Define how to parse a primitive type into your custom scalar.
    fn from_input_value(v: &InputValue) -> Option<i54> {
        v.as_float_value()?.try_into().ok()
    }

    // Define how to parse a string value.
    fn from_str<'a>(value: ScalarToken<'a>) -> juniper::ParseScalarResult<'a, S> {
        <String as juniper::ParseScalarValue<S>>::from_str(value)
    }
}

impl From<i32> for i54 {
    fn from(item: i32) -> Self {
        i54(ux_i54::new(item as i64))
    }
}

impl TryFrom<i64> for i54 {
    type Error = i54Error;
    fn try_from(item: i64) -> Result<Self, Self::Error> {
        let item_i64 = item as i64;
        let item_i54 = i54(ux_i54::new(item_i64));
        if item_i54.as_i64() as i64 != item {
            return Err(i54Error::ConversionFailed);
        }

        Ok(item_i54)
    }
}

impl TryFrom<usize> for i54 {
    type Error = i54Error;
    fn try_from(item: usize) -> Result<Self, Self::Error> {
        let item_i64 = item as i64;
        let item_i54 = i54(ux_i54::new(item_i64));
        if item_i54.as_i64() as usize != item {
            return Err(i54Error::ConversionFailed);
        }

        Ok(item_i54)
    }
}

impl TryFrom<i128> for i54 {
    type Error = i54Error;
    fn try_from(item: i128) -> Result<Self, Self::Error> {
        let item_i128 = item as i128;
        let item_i54 = i54(ux_i54::new(item_i128 as i64));
        if item_i54.as_i64() as i128 != item {
            return Err(i54Error::ConversionFailed);
        }

        Ok(item_i54)
    }
}

impl TryFrom<u128> for i54 {
    type Error = i54Error;
    fn try_from(item: u128) -> Result<Self, Self::Error> {
        let item_u128 = item as u128;
        let item_i54 = i54(ux_i54::new(item_u128 as i64));
        if item_i54.as_i64() as u128 != item {
            return Err(i54Error::ConversionFailed);
        }

        Ok(item_i54)
    }
}

impl TryFrom<f64> for i54 {
    type Error = i54Error;
    fn try_from(item: f64) -> Result<Self, Self::Error> {
        let item_i64 = item as i64;
        let item_i54 = i54(ux_i54::new(item_i64));
        let item_f64 = item_i54.as_i64() as f64;
        if item_f64.to_ne_bytes() != item.to_ne_bytes() {
            return Err(i54Error::ConversionFailed);
        }

        Ok(item_i54)
    }
}

impl i54 {
    pub fn as_i64(&self) -> i64 {
        self.0.into()
    }
}

impl AddAssign for i54 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0);
    }
}

impl Add for i54 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0)
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::FromSql for i54 {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        value
            .as_i64()?
            .try_into()
            .map_err(|_| rusqlite::types::FromSqlError::InvalidType)
    }
}

#[cfg(feature = "rusqlite")]
impl rusqlite::types::ToSql for i54 {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        Ok(rusqlite::types::ToSqlOutput::Owned(
            rusqlite::types::Value::Integer(self.0.into()),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle() {
        // let mut rectangle = Rectangle::new(4, 5);
        let i: i54 = 20_usize.try_into().unwrap();
        assert_eq!(i, 20_i32)
    }
}
