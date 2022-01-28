use sealed::sealed;

#[sealed]
pub trait AsRef<T>
where
    T: ?Sized,
{
    fn as_ref(&self) -> &T;
}

#[sealed]
impl AsRef<str> for String {
    fn as_ref(&self) -> &str {
        &*self
    }
}

fn main() {}
