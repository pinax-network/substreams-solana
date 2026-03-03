use num_enum::TryFromPrimitive;

#[derive(Debug, Clone, Copy, PartialEq, Eq, TryFromPrimitive)]
#[repr(u64)]
pub enum Direction {
    PC2Coin = 1,
    Coin2PC = 2,
}

impl Direction {
    /// Borrow-level view – no heap allocation.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Direction::PC2Coin => "PC2Coin",
            Direction::Coin2PC => "Coin2PC",
        }
    }
}

impl From<Direction> for String {
    fn from(dir: Direction) -> Self {
        dir.as_str().to_owned() // creates an owned `String`
    }
}
