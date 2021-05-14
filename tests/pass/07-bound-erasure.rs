use sealed::sealed;

trait Foo {}
trait Bar {}

#[sealed(erase)]
trait Trait<T>
where
    T: ?Sized + Foo,
{
}

struct Implementor {}

#[sealed(erase)]
impl<T: ?Sized> Trait<T> for Implementor where T: Foo + Bar {}

fn main() {}
