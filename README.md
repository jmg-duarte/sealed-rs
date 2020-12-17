<img src="images/sealed.png" width=100 align="left">

# `#[sealed]`

[<img alt="" src="https://img.shields.io/badge/docs.rs-sealed-success?style=flat-square">](https://docs.rs/sealed)
[<img alt="" src="https://img.shields.io/crates/v/sealed?style=flat-square">](https://crates.io/crates/sealed)

This crate provide a convinient and simple way to implement the sealed trait pattern,
as described in the Rust API Guidelines [[1](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)].



```toml
[dependencies]
sealed = "0.1"
```

## Example

```rust
use sealed::sealed;

#[sealed]
trait T {}

#[sealed]
pub struct A;

impl T for A {}

#[sealed]
pub struct B{
    field_1: i32
}

impl T for B {}

fn main() {
    return
}
```