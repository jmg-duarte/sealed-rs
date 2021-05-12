#![feature(associated_type_bounds)]

use sealed::sealed;

pub trait Foo {}
pub trait Bar {}

#[sealed]
trait Trait<T: ?Sized + Foo> {}

struct Implementor {}

#[sealed]
impl<T: ?Sized> Trait<T> for Implementor where T: Foo + Bar {}

fn main() {}
