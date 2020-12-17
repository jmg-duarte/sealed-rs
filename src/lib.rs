use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote};

#[proc_macro_attribute]
pub fn sealed(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as syn::Item);
    TokenStream::from(match parse_sealed(input) {
        Ok(ts) => {
            println!("{}", ts);
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
        impl private::Sealed for crate::#ident {}
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
