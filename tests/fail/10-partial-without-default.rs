use sealed::sealed;

#[sealed(partial)]
pub trait A {
    #[seal(uncallable)]
    fn sealed();
}

fn main() {}
