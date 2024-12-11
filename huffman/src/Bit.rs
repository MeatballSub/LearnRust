#[derive(Debug, PartialEq)]
pub enum Bit
{
    ZERO,
    ONE,
}

impl From<bool> for Bit
{
    fn from(value: bool) -> Self
    {
        if value
        {
            Bit::ONE
        }
        else
        {
            Bit::ZERO
        }
    }
}
