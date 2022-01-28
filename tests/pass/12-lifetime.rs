use sealed::sealed;

#[sealed]
pub trait Deserialize<'de>: Sized {
    fn deserialize<D>(deserializer: D) -> Option<Self>;
}

#[sealed]
impl<'de> Deserialize<'de> for () {
    fn deserialize<D>(_deserializer: D) -> Option<Self> {
        Some(())
    }
}

fn main() {}
