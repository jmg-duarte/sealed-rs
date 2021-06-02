use sealed::sealed;

#[sealed]
pub trait T {}

#[sealed]
pub trait T1 {}

pub struct A;
pub struct B(i32);

#[sealed]
impl T for A {}
#[sealed]
impl T for B {}

#[sealed]
impl T1 for A {}
#[sealed]
impl T1 for B {}

fn main() {}
