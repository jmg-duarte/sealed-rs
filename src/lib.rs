//! # `#[sealed]`
//!
//! [<img alt="" src="https://img.shields.io/badge/docs.rs-sealed-success?style=flat-square">](https://docs.rs/sealed)
//! [<img alt="" src="https://img.shields.io/crates/v/sealed?style=flat-square">](https://crates.io/crates/sealed)
//!
//! This crate provides a convenient and simple way to implement the sealed trait pattern,
//! as described in the Rust API Guidelines [[1](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)].
//!
//! ```toml
//! [dependencies]
//! sealed = "0.1"
//! ```
//!
//! ## Example
//!
//! In the following code structs `A` and `B` implement the sealed trait `T`,
//! the `C` struct, which is not sealed, will error during compilation.
//!
//! You can see a demo in [`demo/`](demo/).
//!
//! ```rust,compile_fail
//! use sealed::sealed;
//!
//! #[sealed]
//! trait T {}
//!
//! #[sealed]
//! pub struct A;
//!
//! impl T for A {}
//!
//! #[sealed]
//! pub struct B;
//!
//! impl T for B {}
//!
//! pub struct C;
//!
//! impl T for C {} // compile error
//! ```
//!
//! ## Details
//!
//! The macro generates a `private` module when attached to a `trait`
//! (this raises the limitation that the `#[sealed]` macro can only be added to a single trait per module),
//! when attached to a `struct` the generated code simply implements the sealed trait for the respective structure.
//!
//!
//! ### Expansion
//!
//! ```rust
//! // #[sealed]
//! // trait T {}
//! trait T: private::Sealed {}
//! mod private {
//!     pub trait Sealed {}
//! }
//!
//! // #[sealed]
//! // pub struct A;
//! pub struct A;
//! impl private::Sealed for A {}
//! ```

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn sealed(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Item);
    TokenStream::from(match parse_sealed(input) {
        Ok(ts) => {
            ts
        },
        Err(err) => err.to_compile_error(),
    })
}

fn parse_sealed(item: syn::Item) -> syn::Result<proc_macro2::TokenStream> {
    match item {
        syn::Item::Struct(item_struct) => parse_sealed_struct(item_struct),
        syn::Item::Trait(item_trait) => parse_sealed_trait(item_trait),
        _ => Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected struct or trait",
        )),
    }
}

// Care for https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1#procedural-macros-support-renaming-the-crate
fn parse_sealed_struct(strct: syn::ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let ident = &strct.ident;
    Ok(quote!(
        #strct
        impl private::Sealed for #ident {}
    ))
}

// Care for https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1#procedural-macros-support-renaming-the-crate
fn parse_sealed_trait(mut trt: syn::ItemTrait) -> syn::Result<proc_macro2::TokenStream> {
    trt.supertraits.push(parse_quote!(private::Sealed));
    Ok(quote!(
        #trt
        mod private {
            pub trait Sealed {}
        }
    ))
}
