use sealed::sealed;

#[sealed]
pub trait r#Pub<V> {}

#[sealed]
impl<T> r#Pub<Option<T>> for T {}

#[sealed]
impl<T> r#Pub<Option<T>> for Option<T> {}

fn main() {}
