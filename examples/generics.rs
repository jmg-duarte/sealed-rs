// Test provided in #4
// https://github.com/jmg-duarte/sealed-rs/issues/4

use sealed::sealed;

#[sealed]
pub trait Set<V> {}

#[sealed]
impl<T> Set<Option<T>> for T {}

#[sealed]
impl<T> Set<Option<T>> for Option<T> {}

fn main() {}
