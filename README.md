<img src="images/sealed.png" width=100 align="left">

# `#[sealed]`

[<img alt="" src="https://img.shields.io/badge/docs.rs-sealed-success?style=flat-square">](https://docs.rs/sealed)
[<img alt="" src="https://img.shields.io/crates/v/sealed?style=flat-square">](https://crates.io/crates/sealed)

This crate provides a convenient and simple way to implement the sealed trait pattern,
as described in the Rust API Guidelines [[1](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)].

```toml
[dependencies]
sealed = "0.2.1"
```

## Example

In the following code structs `A` and `B` implement the sealed trait `T`,
the `C` struct, which is not sealed, will error during compilation.

Examples are available in [`examples/`](examples/), you can also see a demo in [`demo/`](demo/).

```rust
use sealed::sealed;

#[sealed]
trait T {}

pub struct A;

#[sealed]
impl T for A {}

pub struct B;

#[sealed]
impl T for B {}

pub struct C;

impl T for C {} // compile error

fn main() {
    return
}
```

## Attributes

This is the list of attributes that can be used along `#[sealed]`:
- `#[sealed]`: the main attribute macro, without attribute parameters.
- `#[sealed(erase)]`: this option turns on bound erasure. This is useful when using the `#[sealed]` macro inside a function.
For an example, see [`bound-erasure-fn`](tests/pass/08-bound-erasure-fn.rs).

## Details

The `#[sealed]` attribute can be attached to either a `trait` or an `impl`.
It supports:
- Several traits per module
- Generic parameters
- Foreign types
- Blanket `impl`s


## Expansion

### Input

```rust
use sealed::sealed;

#[sealed]
pub trait T {}

pub struct A;
pub struct B(i32);

#[sealed]
impl T for A {}
#[sealed]
impl T for B {}

fn main() {
    return;
}
```

### Expanded

```rust
use sealed::sealed;

pub(crate) mod __seal_t {
    pub trait Sealed {}
}
pub trait T: __seal_t::Sealed {}

pub struct A;
pub struct B(i32);

impl __seal_t::Sealed for A {}
impl T for A {}

impl __seal_t::Sealed for B {}
impl T for B {}

fn main() {
    return;
}
```