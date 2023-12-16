use sealed::sealed;

#[sealed]
pub trait A {
	#[seal(callable)]
	fn b() {}
}

fn main() {}
