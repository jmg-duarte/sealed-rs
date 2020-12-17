<img src="images/sealed.png" width=100 align="left">

# `#[sealed]`

[<img alt="" src="https://img.shields.io/badge/docs.rs-sealed-success?style=flat-square">](https://docs.rs/sealed)
[<img alt="" src="https://img.shields.io/crates/v/sealed?style=flat-square">](https://crates.io/crates/sealed)

This crate provides a convenient and simple way to implement the sealed trait pattern,
as described in the Rust API Guidelines [[1](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)].

```toml
[dependencies]
sealed = "0.1"
```

## Example

In the following code structs `A` and `B` implement the sealed trait `T`,
the `C` struct, which is not sealed, will error during compilation.

You can see a demo in [`demo/`](demo/).

```rust
use sealed::sealed;

#[sealed]
trait T {}

#[sealed]
pub struct A;

impl T for A {}

#[sealed]
pub struct B;

impl T for B {}

pub struct C;

impl T for C {} // compile error

fn main() {
    return
}
```

## Details

The macro generates a `private` module when attached to a `trait`
(this raises the limitation that the `#[sealed]` macro can only be added to a single trait per module),
when attached to a `struct` the generated code simply implements the sealed trait for the respective structure.


### Expansion

```rust
// #[sealed]
// trait T {}
trait T: private::Sealed {}
mod private {
    trait Sealed {}
}

// #[sealed]
// pub struct A;
pub struct A;
impl private::Sealed for A {}
```