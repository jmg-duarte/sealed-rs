#![feature(associated_type_bounds)]

use sealed::sealed;

// trait Foo {}

// #[sealed]
// trait Trait<T: Foo> {}

// trait Unrelated<T> {}

// // #[sealed]
// impl<T> Unrelated<T> for () {}

fn main() {
    trait Foo {}
    trait Bar {}

    #[sealed(erase)]
    trait Trait<T> where T: ?Sized + Foo{}

    struct Implementor {}

    #[sealed(erase)]
    impl<T: ?Sized> Trait<T> for Implementor where T: Foo + Bar {}
}
