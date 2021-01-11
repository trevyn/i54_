# i54 [![Crates.io](https://img.shields.io/crates/v/i54.svg)](https://crates.io/crates/i54)

A 54-bit signed integer abstraction. Created for easier interop with GraphQL and Javascript, which don't have proper i64-compatible primitives.

## Premise

Both Javascript and GraphQL natively represent large (over 32-bit) numbers as signed double-precision floating-point values, and do not have primitives representing large integers.

This presents a problem when wanting to represent an integer larger than a `u32` in Rust, while maintaining type interop with Javascript and GraphQL.

To make programmer intent clear, we provide an `i54` type that should behave similarly to a Rust primitive type, and is intended to represent values that would fit in a hypothetical 54-bit signed integer primitive.

`i54` is the largest signed integer for which all possible values are encoded exactly by an [IEEE 754 double-precision floating-point representation](https://en.wikipedia.org/wiki/Double-precision_floating-point_format). (Encoded roughly as 52 bits in the mantissa, 1 bit derived from the exponent bits, and 1 sign bit.)

## Exciting Trait Implementations

- `serde` - `Serialize`, `Deserialize`
- `rusqlite` _(optional)_ - `FromSql`, `ToSql`
- `juniper` _(optional)_ - `GraphQLScalar`

## Usage

```toml
i54 = {version = "0.0.1", features = ["rusqlite", "juniper"]}
```

```rust
use i54::i54;

fn main() {
 let x: i54 = 1;
 x += 1;
 assert!(x == 2);
}
```

### License: MIT OR Apache-2.0
