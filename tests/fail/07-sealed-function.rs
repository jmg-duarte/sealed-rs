pub mod inner {
    use sealed::sealed;

    #[sealed]
    pub trait PartialSealed {
        #[seal(uncallable)]
        fn a();

        fn b();
    }

    pub struct A;

    #[sealed]
    impl PartialSealed for A {
        #[seal(uncallable)]
        fn a() {}

        fn b() {}
    }
}

use crate::inner::PartialSealed;

fn main() {
    inner::A::a();
    inner::A::a(Token);
    inner::A::b();
}
