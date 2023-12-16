use sealed::sealed;

#[sealed(partial)]
pub trait A {
	#[seal]
	fn sealed() {}
}

#[sealed(partial)]
pub trait B {
	fn not_sealed();
}

fn main() {}
