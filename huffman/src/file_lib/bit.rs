#[derive(Debug, PartialEq)]
pub enum Bit
{
    Zero,
    One,
}

impl From<bool> for Bit
{
    fn from(value: bool) -> Self
    {
        if value
        {
            Bit::One
        }
        else
        {
            Bit::Zero
        }
    }
}
