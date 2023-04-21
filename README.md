<img src="images/sealed.png" width=100 align="left">

# `#[sealed]`

[<img alt="" src="https://img.shields.io/badge/docs.rs-sealed-success?style=flat-square">](https://docs.rs/sealed)
[<img alt="" src="https://img.shields.io/crates/v/sealed?style=flat-square">](https://crates.io/crates/sealed)
<img alt="MSRV 1.56.0" src="https://img.shields.io/badge/msrv-1.56.0-blue?style=flat-square">

This crate provides a convenient and simple way to implement the sealed trait pattern,
as described in the Rust API Guidelines [[1](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)].

```toml
[dependencies]
sealed = "0.5"
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
```

## Arguments

This is the list of arguments that can be used in a `#[sealed]` attribute:

- `#[sealed(erase)]`: turns on trait bounds erasure. This is useful when using the `#[sealed]` macro inside a function. For an example, see [`bound-erasure-fn`](examples/bound-erasure-fn.rs) example.

- `#[sealed(pub(crate))]` or `#[sealed(pub(in some::path))]`: allows to tune visibility of the generated sealing module (the default one is private). This useful when the trait and its impls are defined in different modules. For an example, see [`nesting`](examples/nesting.rs) example. **Notice**, that just `pub` is disallowed as breaks the whole idea of sealing.


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
