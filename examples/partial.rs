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

    fn not_sealed();
}

pub struct Email(String);

pub struct User {
    name: String,
    email: Email,
    age: u8,
}

pub struct Impl;

#[sealed]
impl A for Impl {
    fn not_sealed() {}
}

#[sealed(partial)]
pub trait B {
    #[seal(callable)]
    fn no_default(_email: Email) {}

    fn hello();
}

#[sealed]
impl B for Impl {
    #[seal(callable)]
    fn no_default(_email: Email) {}

    fn hello() {}
}

#[sealed]
pub trait NoPartial {
    #[seal(uncallable)]
    fn has_default() {}

    fn no_default();
}

#[sealed]
impl NoPartial for Impl {
    fn no_default() {}
}

fn main() {}
