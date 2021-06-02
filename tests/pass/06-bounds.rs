use sealed::sealed;

trait Foo {}

#[sealed(erase)]
trait Trait<T: ?Sized + Foo> {}

fn main() {}
