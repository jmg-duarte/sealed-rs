use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn sealed(_args: TokenStream, input: TokenStream) -> TokenStream {
    let mut output = input.clone();
    let input = parse_macro_input!(input as syn::Item);
    println!("{:#?}", input);
    match parse_sealed(input) {
        Ok(ts) => output.extend(proc_macro::TokenStream::from(ts)),
        Err(err) => output.extend(proc_macro::TokenStream::from(err.to_compile_error())),
    }
    output
}

fn parse_sealed(item: syn::Item) -> syn::Result<proc_macro2::TokenStream> {
    if let syn::Item::Struct(item_struct) = item {
        parse_sealed_struct(item_struct)
    } else {
        Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "expected struct",
        ))
    }
}

// Care for https://gist.github.com/Koxiaet/8c05ebd4e0e9347eb05f265dfb7252e1#procedural-macros-support-renaming-the-crate
fn parse_sealed_struct(strct: syn::ItemStruct) -> syn::Result<proc_macro2::TokenStream> {
    let ident = strct.ident;
    Ok(quote!(
        impl private::Sealed for crate::#ident {}
    ))
}
