use sealed::sealed;

#[sealed(partial)]
pub trait A {
	#[seal(callable)]
	fn has_default(
		Email(_email): Email,
		User {
			name: _name,
			email: Email(_email2),
			age: _age,
		}: User,
	) {
	}
}

pub struct Email(String);

pub struct User {
	name: String,
	email: Email,
	age: u8,
}

pub struct Impl;

#[sealed]
impl A for Impl {}

#[sealed(partial)]
pub trait B {
	#[seal(callable)]
	fn no_default(email: Email);
}

#[sealed]
impl B for Impl {
	#[seal(callable)]
	fn no_default(_email: Email) {}
}

#[sealed]
pub trait NoPartial {
	#[seal]
	fn has_default() {}

	fn no_default();
}

#[sealed]
impl NoPartial for Impl {
	fn no_default() {}
}

fn main() {}
