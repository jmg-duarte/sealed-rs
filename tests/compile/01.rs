use sealed::sealed;

#[sealed]
pub struct A;
#[sealed]
pub struct B{
    field_1: i32
}

#[sealed]
trait T {}

impl T for A {}
impl T for B {}

fn main() {
    return
}
