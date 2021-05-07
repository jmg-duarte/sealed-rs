// Test provided in #4
// https://github.com/jmg-duarte/sealed-rs/issues/4

use sealed::sealed;

#[sealed]
pub trait Set<V> {}
// pub trait Set<V>: __seal_for_set::Sealed<V> {}
// mod __seal_for_set {
//     pub trait Sealed<V> {}
// }

#[sealed]
impl<T> Set<Option<T>> for T {}
// impl<T> __seal_for_set::Sealed<Option<T>> for T {}

#[sealed]
impl<T> Set<Option<T>> for Option<T> {}
// impl<T> __seal_for_set::Sealed<Option<T>> for Option<T> {}

fn main() {}
