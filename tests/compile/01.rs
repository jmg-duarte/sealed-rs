use sealed::sealed;

#[sealed]
pub struct A;
#[sealed]
pub struct B {
    field_1: i32,
}
pub struct C;

#[sealed]
trait T {}

impl T for A {}
impl T for B {}
impl T for C {}

fn main() {
    return;
}
