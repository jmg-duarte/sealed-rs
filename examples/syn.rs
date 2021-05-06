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
// pub trait AsSpan: private::Sealed {
//     fn as_span(&self) -> Span;
// }
// mod private {
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
// impl private::Sealed for Span {}

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
// impl<T: Spanned> private::Sealed for &T {}

fn main() {}
