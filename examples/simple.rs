use sealed::sealed;

#[sealed]
pub trait T {}

pub struct A;
pub struct B {
    field_1: i32,
}

#[sealed]
impl T for A {}
#[sealed]
impl T for B {}


fn main() {
    return;
}
