use sealed::sealed;

#[sealed]
pub trait FromArray<T, const N: usize> {
    fn from(arr: [T; N]) -> Self;
}

#[sealed]
impl<T, const N: usize> FromArray<T, N> for Vec<T> {
    fn from(arr: [T; N]) -> Self {
        IntoIterator::into_iter(arr).collect()
    }
}

fn main() {}
