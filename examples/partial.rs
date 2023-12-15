use sealed::sealed;

#[sealed]
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

pub struct AImpl;

#[sealed]
impl A for AImpl {}

fn main() {}
