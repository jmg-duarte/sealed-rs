use sealed::sealed;

mod lets {
    pub mod attempt {
        pub mod some {
            pub mod nesting {
                use sealed::sealed;
                #[sealed]
                pub trait T {}
            }
        }
    }
}

pub struct A;

#[sealed]
impl lets::attempt::some::nesting::T for A {}

fn main() {}
