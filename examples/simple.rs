use sealed::sealed;

#[sealed]
pub trait T {}

pub struct A;
pub struct B(i32);

#[sealed]
impl T for A {}
#[sealed]
impl T for B {}

fn main() {}
