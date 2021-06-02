use sealed::sealed;

pub enum Enum {}

#[sealed(erase, pub(crate))]
pub trait Sealer {}

#[sealed]
impl Sealer for Enum {}

fn main() {}
