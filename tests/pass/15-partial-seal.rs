use sealed::sealed;

#[sealed(partial)]
pub trait A {
	#[seal(callable)]
	fn sealed() {}

	fn normal();
}

fn main() {}
