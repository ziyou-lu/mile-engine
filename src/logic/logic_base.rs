use crate::common::error::Error;

pub trait LogicBase {
    fn init(&self) -> Result<(), Error>;

    fn shut(&self) -> Result<(), Error>;
}