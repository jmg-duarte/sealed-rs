use sealed::sealed;

pub enum Enum {}

#[sealed]
pub trait Sealer {}

#[sealed]
impl Sealer for Enum {}

fn main() {}
