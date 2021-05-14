// Example provided in #4
// https://github.com/jmg-duarte/sealed-rs/issues/4

use proc_macro2::*;
use sealed::sealed;
use syn::spanned::Spanned;

#[sealed]
pub trait AsSpan {
    fn as_span(&self) -> Span;
}
// expands to:
// pub trait AsSpan: __seal_as_span::Sealed {
//     fn as_span(&self) -> Span;
// }
// mod __seal_as_span {
//     pub trait Sealed {}
// }

#[sealed]
impl AsSpan for Span {
    fn as_span(&self) -> Self {
        *self
    }
}
// expands to:
// impl AsSpan for Span {  // foreign type, cannot place #[sealed]
//     fn as_span(&self) -> Self { *self }
// }
// impl __seal_as_span::Sealed for Span {}

#[sealed]
impl<T: Spanned> AsSpan for &T {
    fn as_span(&self) -> Span {
        self.span()
    }
}
// expands to:
// impl<T: Spanned> AsSpan for &T {
//     fn as_span(&self) -> Span { self.span() }
// }
// impl<T: Spanned> __seal_as_span::Sealed for &T {}

fn main() {}
