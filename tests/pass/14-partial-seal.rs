use sealed::sealed;

#[sealed]
pub trait PartialSealed {
	#[seal]
	fn default(_value: String) {}

	#[seal]
	fn no_default(value: u8);
}

pub struct A;

#[sealed]
impl PartialSealed for A {
	#[seal]
	fn no_default(_value: u8) {}
}

pub struct B;

#[sealed]
impl PartialSealed for B {
	#[seal]
	fn default(_value: String) {}

	#[seal]
	fn no_default(_value: u8) {}
}

fn main() {}
