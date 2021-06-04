use sealed::sealed;

pub struct A;

pub struct B {
    field_1: i32,
}

pub struct C;

#[sealed]
trait T {}

#[sealed]
impl T for A {}

#[sealed]
impl T for B {}

impl T for C {}

fn main() {}
