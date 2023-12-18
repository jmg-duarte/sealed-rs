use sealed::sealed;

#[sealed(partial)]
pub trait A {
    #[seal(uncallable)]
    fn sealed() {}
}

#[sealed(partial)]
pub trait B {
    fn not_sealed();
}

fn main() {}
