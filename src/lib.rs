//! # `#[sealed]`
//!
//! [<img alt="" src="https://img.shields.io/badge/docs.rs-sealed-success?style=flat-square">](https://docs.rs/sealed)
//! [<img alt="" src="https://img.shields.io/crates/v/sealed?style=flat-square">](https://crates.io/crates/sealed)
//! <img alt="MSRV 1.51.0" src="https://img.shields.io/badge/msrv-1.51.0-blue?style=flat-square">
//!
//! This crate provides a convenient and simple way to implement the sealed trait pattern,
//! as described in the Rust API Guidelines [[1](https://rust-lang.github.io/api-guidelines/future-proofing.html#sealed-traits-protect-against-downstream-implementations-c-sealed)].
//!
//! ```toml
//! [dependencies]
//! sealed = "0.5"
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
//! # use sealed::sealed;
//! #
//! #[sealed]
//! trait T {}
//!
//! pub struct A;
//! #[sealed]
//! impl T for A {}
//!
//! pub struct B;
//! #[sealed]
//! impl T for B {}
//!
//! pub struct C;
//! impl T for C {} // compile error
//! ```
//!
//! ## Details
//!
//! The attribute generates a private uniquely named module when attached to a
//! trait definition, when attached to an `impl` block the generated code simply
//! implements the sealed trait for the respective type.
//!
//! ```rust,ignore
//! // #[sealed]
//! // trait T {}
//! trait T: __seal_t::Sealed {}
//! mod __seal_t {
//!     pub trait Sealed {}
//! }
//!
//! pub struct A;
//!
//! // #[sealed]
//! // impl T for A {}
//! impl T for A {}
//! impl __seal_t::Sealed for A {}
//! ```
//!
//! ## Arguments
//!
//! The expanded code may be customized with the following attribute arguments.
//!
//! ### `erase`
//!
//! Turns on trait bounds erasure. This is useful when using the `#[sealed]`
//! attribute inside a function. By default, all the bounds are propagated to
//! the generated `Sealed` trait.
//!
//! ```rust,ignore
//! // #[sealed(erase)]
//! // trait Trait<T: ?Sized + Default> {}
//! trait Trait<T: ?Sized + Default>: __seal_trait::Sealed<T> {}
//! mod __seal_trait {
//!     pub trait Sealed<T> {}
//! }
//! ```
//!
//! ### `pub(crate)` or `pub(in some::path)`
//!
//! Allows to tune visibility of the generated sealing module (the default one
//! is private). This useful when the trait and its impls are defined in
//! different modules.
//!
//! ```rust
//! # use sealed::sealed;
//! #
//! mod lets {
//!     pub mod attempt {
//!         pub mod some {
//!             pub mod nesting {
//! #               use sealed::sealed;
//!                 #[sealed(pub(in super::super::super::super))]
//!                 pub trait T {}
//!             }
//!         }
//!     }
//! }
//!
//! pub struct A;
//! #[sealed]
//! impl lets::attempt::some::nesting::T for A {}
//! ```
//!
//! Notice, that just `pub` is disallowed as breaks the whole idea of sealing.
//!
//! ```rust,compile_fail
//! # use sealed::sealed;
//! #
//! #[sealed(pub)] // compile error
//! trait T {}
//!
//! pub struct A;
//! #[sealed]
//! impl T for A {}
//! ```

use std::fmt;

use heck::ToSnakeCase as _;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
	ext::IdentExt,
	parse::{Parse, ParseStream},
	parse_macro_input, parse_quote,
	spanned::Spanned,
	token,
};

#[proc_macro_attribute]
pub fn sealed(args: TokenStream, input: TokenStream) -> TokenStream {
	match parse_macro_input!(input) {
		syn::Item::Impl(mut item_impl) => parse_sealed_impl(&mut item_impl),
		syn::Item::Trait(item_trait) => {
			Ok(parse_sealed_trait(item_trait, parse_macro_input!(args)))
		}
		_ => Err(syn::Error::new(Span::call_site(), "expected impl or trait")),
	}
	.unwrap_or_else(|e| e.to_compile_error())
	.into()
}

// Care for https://gist.github.com/Kestrer/8c05ebd4e0e9347eb05f265dfb7252e1#procedural-macros-support-renaming-the-crate
fn parse_sealed_trait(mut item_trait: syn::ItemTrait, args: TraitArguments) -> TokenStream2 {
	let trait_ident = &item_trait.ident.unraw();
	let trait_generics = &item_trait.generics;
	let seal = seal_name(trait_ident);
	let vis = &args.visibility;

	let (_, ty_generics, where_clause) = trait_generics.split_for_impl();
	item_trait
		.supertraits
		.push(parse_quote!( #seal::Sealed #ty_generics ));

	let mut wrappers = Vec::<syn::TraitItem>::new();

	let mut all_implementable = true;
	let mut none_implementable = true;

	for item in &mut item_trait.items {
		if let syn::TraitItem::Fn(item) = item {
			let input = match parse_function_arguments(&item.attrs) {
				Ok(input) => input,
				Err(err) => {
					return err
						.into_compile_error()
						.into()
				}
			};

			let Some((attr, input)) = input else {
				none_implementable = false;
				continue;
			};

			let attr = attr.clone();

			item.attrs
				.retain(|x| x != &attr);

			match parse_function_definition(trait_ident, input, item) {
				Ok(Some(_)) if !args.partial => {
					return syn::Error::new(
						attr.span(),
						"Sealing a function from implementation but allowing it to be called is already done by sealing the trait itself. \
							If you want to seal this function, but not others, \
							consider adding `partial` to the `sealed` attr on the trait: `#[sealed(partial)]`"
						)
						.into_compile_error()
						.into()
				}
				Ok(_) if args.partial && item.default.is_none() => {
					return syn::Error::new(
						attr.span(), 
						"This function is sealed from implementation, \
							but it does not have a default implementation. \
							This effectively seals the entire trait, \
							which would be clearer to do by not having the trait seal be partial."
						).into_compile_error().into()
				}
				Ok(Some(wrapper)) => {
					all_implementable = false;
					wrappers.push(wrapper.into())
				},
				Ok(None) => all_implementable = false,
				Err(err) => {
					return err
						.into_compile_error()
						.into()
				}
			}
		}
	}

	if args.partial && all_implementable {
		return syn::Error::new(item_trait.ident.span(), "This trait is partially sealed, \
			however none of its methods are sealed, so it does nothing. \
			Either seal a function or remove `partial` from the `sealed` attribute.").to_compile_error().into();
	}

	if args.partial && none_implementable {
		return syn::Error::new(item_trait.ident.span(), "This trait is partially sealed, \
		however none of its methods are implementable, so the `partial` argument does nothing.").into_compile_error().into();
	}

	item_trait
		.items
		.extend(wrappers);

	let mod_code = if args.erased {
		let lifetimes = trait_generics.lifetimes();
		let const_params = trait_generics.const_params();
		let type_params = trait_generics
			.type_params()
			.map(|syn::TypeParam { ident, .. }| -> syn::TypeParam {
				parse_quote!( #ident : ?Sized )
			});

		// Token here is used as an argument to a function in order to seal it.
		quote! {
			pub trait Sealed< #(#lifetimes ,)* #(#type_params ,)* #(#const_params ,)* > {}
			pub struct Token;
		}
	} else {
		// `trait_generics` does not output its where clause when tokenized (due
		// to supertraits in the middle). So we output them separately.
		quote! {
			use super::*;
			pub trait Sealed #trait_generics #where_clause {}
			pub struct Token;
		}
	};

	quote! {
		#[automatically_derived]
		#vis mod #seal {
			#mod_code
		}
		#item_trait
	}
}

fn parse_sealed_impl(item_impl: &mut syn::ItemImpl) -> syn::Result<TokenStream2> {
	let impl_trait = item_impl
		.trait_
		.as_ref()
		.ok_or_else(|| {
			syn::Error::new_spanned(item_impl.clone(), "missing implementation trait")
		})?;

	let mut sealed_path = impl_trait.1.segments.clone();

	// since `impl for ...` is not allowed, this path will *always* have at least length 1
	// thus both `first` and `last` are safe to unwrap
	let syn::PathSegment { ident, arguments } = sealed_path
		.pop()
		.unwrap()
		.into_value();
	let seal = seal_name(ident.unraw());
	sealed_path.push(parse_quote!( #seal ));
	sealed_path.push(parse_quote!(Sealed));

	let self_type = &item_impl.self_ty;

	// Only keep the introduced params (no bounds), since
	// the bounds may break in the `#seal` submodule.
	let (trait_generics, _, where_clauses) = item_impl
		.generics
		.split_for_impl();

	for item in &mut item_impl.items {
		if let syn::ImplItem::Fn(item) = item {
			let input = parse_function_arguments(&item.attrs)?;

			let Some((attr, input)) = input else {
				continue;
			};

			let attr = attr.clone();

			item.attrs
				.retain(|x| x != &attr);

			parse_function_implementation(&ident, input, item)?;
		}
	}

	Ok(quote! {
		#[automatically_derived]
		impl #trait_generics #sealed_path #arguments for #self_type #where_clauses {}
		#item_impl
	})
}

/// Constructs [`syn::Ident`] of a sealing module name.
fn seal_name<D: fmt::Display>(seal: D) -> syn::Ident {
	format_ident!(
		"__seal_{}",
		&seal
			.to_string()
			.to_snake_case()
	)
}

/// Constructs [`syn::Ident`] of a function wrapped by another function
fn seal_function_name<D: fmt::Display>(seal: D) -> syn::Ident {
	format_ident!("_{}", &seal.to_string())
}

fn parse_function_arguments(
	attrs: &[syn::Attribute],
) -> Result<Option<(&syn::Attribute, FunctionArguments)>, syn::Error> {
	for attr in attrs {
		if attr.path().is_ident("seal") {
			let value = if let syn::Meta::List(list) = &attr.meta {
				syn::parse(list.tokens.clone().into())?
			} else {
				FunctionArguments::default()
			};

			return Ok(Some((attr, value)));
		}
	}

	Ok(None)
}

/// Seal a specific function (to avoid people overwriting behaviour of
/// specific functions, or even calling them)
fn parse_function_definition(
	trait_ident: &syn::Ident,
	args: FunctionArguments,
	function: &mut syn::TraitItemFn,
) -> Result<Option<syn::TraitItemFn>, syn::Error> {
	let wrapper_sig = function.sig.clone();
	let seal = seal_name(trait_ident);

	function
		.sig
		.inputs
		.push(parse_quote!(_token: #seal::Token));

	let mut value = None;

	if args.callable {
		function.sig.ident = seal_function_name(&function.sig.ident);
		let inner_name = &function.sig.ident;

		let args = wrapper_sig
			.inputs
			.iter()
			.map(|input| match input {
				syn::FnArg::Receiver(_) => quote!(self),
				syn::FnArg::Typed(input) => {
					let pat = &input.pat;
					quote!(#pat)
				}
			});

		value = Some(parse_quote!(#wrapper_sig {
			<Self as #trait_ident>::#inner_name(#(#args,)* #seal::Token)
		}));
	}
	Ok(value)
}

fn parse_function_implementation(
	trait_ident: &syn::Ident,
	args: FunctionArguments,
	function: &mut syn::ImplItemFn,
) -> Result<(), syn::Error> {
	let seal = seal_name(trait_ident);

	function
		.sig
		.inputs
		.push(parse_quote!(_token: #seal::Token));

	if args.callable {
		function.sig.ident = seal_function_name(&function.sig.ident);
	}

	Ok(())
}

/// Arguments accepted by `#[sealed]` attribute when placed on a trait
/// definition.
struct TraitArguments {
	/// `erase` argument indicating whether trait bounds erasure should be used.
	///
	/// Default is `false`.
	erased: bool,

	/// `pub` argument defining visibility of the generated sealing module.
	///
	/// Default is [`syn::Visibility::Inherited`].
	visibility: syn::Visibility,

	/// `partial` indicates that the trait itself should not be sealed, but some of the functions
	/// are sealed
	partial: bool,
}

impl Default for TraitArguments {
	fn default() -> Self {
		Self {
			erased: false,
			visibility: syn::Visibility::Inherited,
			partial: false,
		}
	}
}

impl Parse for TraitArguments {
	fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
		let mut out = Self::default();

		while !input.is_empty() {
			let ident = syn::Ident::parse_any(&input.fork())?;

			match ident.to_string().as_str() {
				"erase" => {
					syn::Ident::parse_any(input)?;
					out.erased = true;
				}

				"pub" => {
					out.visibility = input.parse()?;
					if matches!(out.visibility, syn::Visibility::Public(_)) {
						return Err(syn::Error::new(
							out.visibility.span(),
							"`pub` visibility breaks the seal as allows to use \
                             it outside its crate.\n\
                             Consider tightening the visibility (e.g. \
                             `pub(crate)`) if you actually need sealing.",
						));
					}
				}

				"partial" => {
					syn::Ident::parse_any(input)?;
					out.partial = true;
				}

				unknown => {
					return Err(syn::Error::new(
						ident.span(),
						format!("unknown `{}` attribute argument", unknown),
					))
				}
			}

			if input
				.lookahead1()
				.peek(token::Comma)
				.then(|| input.parse::<token::Comma>())
				.transpose()?
				.is_none() && !input.is_empty()
			{
				return Err(syn::Error::new(ident.span(), "expected followed by `,`"));
			}
		}

		Ok(out)
	}
}

/// Arguments accepted by `#[seal]` attribute when placed on functions in a (partially) sealed trait
struct FunctionArguments {
	/// Determines whether the sealed function is wrapped by a public function that is callable
	callable: bool,
}

impl Default for FunctionArguments {
	fn default() -> Self {
		FunctionArguments { callable: false }
	}
}

impl Parse for FunctionArguments {
	fn parse(input: ParseStream) -> syn::Result<Self> {
		let mut out = FunctionArguments::default();
		let ident = syn::Ident::parse(&input.fork())?;

		match ident.to_string().as_str() {
			"callable" => {
				syn::Ident::parse(input)?;
				out.callable = true
			}
			unknown => {
				return Err(syn::Error::new(
					ident.span(),
					format!("unknown `{}` attribute argument", unknown),
				))
			}
		}

		Ok(out)
	}
}
