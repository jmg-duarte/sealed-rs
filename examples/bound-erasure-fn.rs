use sealed::sealed;

fn main() {
    trait Foo {}
    trait Bar {}

    #[sealed(erase)]
    trait Trait<T: ?Sized + Foo> {}

    struct Implementor {}

    #[sealed(erase)]
    impl<T: ?Sized> Trait<T> for Implementor where T: Foo + Bar {}
}
