use sealed::sealed;

#[sealed]
pub struct A;
// impl private::Sealed for crate::A {}
#[sealed]
pub struct B;
// impl private::Sealed for crate::B {}

// #[derive(Sealed)]
trait T : private::Sealed {}
// generates
mod private {
    pub trait Sealed {}
}

impl T for A {}
impl T for B {}

fn main() {
    return
}
