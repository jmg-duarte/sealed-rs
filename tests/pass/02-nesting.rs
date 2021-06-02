use sealed::sealed;

mod lets {
    pub mod attempt {
        pub mod some {
            pub mod nesting {
                use sealed::sealed;
                #[sealed(pub(in super::super::super::super))]
                pub trait T {}
            }
        }
    }
}

pub struct A;

pub struct B {
    field_1: i32,
}

#[sealed]
impl lets::attempt::some::nesting::T for A {}

#[sealed]
impl lets::attempt::some::nesting::T for B {}

fn main() {}
