use sealed::sealed;

#[sealed]
pub trait CopyIterator: Iterator
where
    Self::Item: Copy,
{
}

fn main() {}
