use sealed::sealed;

#[sealed]
pub trait FromArray<T: Copy, const N: usize> {
    fn from(arr: [T; N]) -> Self;
}

#[sealed]
impl<T: Copy, const N: usize> FromArray<T, N> for Vec<T> {
    fn from(arr: [T; N]) -> Self {
        let mut v = Vec::with_capacity(N);
        for i in 0..N {
            v[i] = arr[i];
        }
        v
    }
}

fn main() {}
