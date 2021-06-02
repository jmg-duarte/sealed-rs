use sealed::sealed;

#[sealed(erased)]
pub trait T {}

pub struct A;

#[sealed]
impl T for A {}

fn main() {}
