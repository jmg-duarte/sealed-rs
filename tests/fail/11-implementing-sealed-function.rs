mod inner {
    use sealed::sealed;

    #[sealed(partial)]
    pub trait A {
        #[seal(uncallable)]
        fn sealed() {}

        fn unsealed();
    }
}

use inner::A;

struct B;

impl A for B {
    fn sealed() {}
    fn unsealed() {}
}

struct C;

impl A for C {
    fn sealed(_token: Token) {}
    fn unsealed() {}
}

fn main() {
    B::sealed();
    B::unsealed();
}
