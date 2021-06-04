use sealed::sealed;

mod lets {
    pub mod attempt {
        pub mod some {
            pub mod nesting {
                use sealed::sealed;
                #[sealed(pub(crate))]
                pub trait LongerSnakeCaseType {}
            }
        }
    }
}

pub struct A;

pub struct B(i32);

#[sealed]
impl lets::attempt::some::nesting::LongerSnakeCaseType for A {}

#[sealed]
impl lets::attempt::some::nesting::LongerSnakeCaseType for B {}

fn main() {}
