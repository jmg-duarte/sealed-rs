use sealed::sealed;

#[sealed(partial)]
pub trait A {
	#[seal]
	fn sealed();
}

fn main() {}
