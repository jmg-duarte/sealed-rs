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
    fn unsealed() {}
}

fn main() {
    B::sealed();
    B::sealed(Token);
    B::unsealed();
}
