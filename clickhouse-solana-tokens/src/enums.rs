use std::fmt;

/// --- TokenStandard --------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStandard {
    Native,
    ClassicSplToken,
    SplToken2022,
}

impl fmt::Display for TokenStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenStandard::Native => write!(f, "Native"),
            TokenStandard::ClassicSplToken => write!(f, "Classic SPL Token"),
            TokenStandard::SplToken2022 => write!(f, "SPL Token-2022"),
        }
    }
}
