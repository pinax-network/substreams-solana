use std::fmt;

/// --- TokenStandard --------------------------------------------------------
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenStandard {
    ClassicSplToken,
    SplToken2022,
    Native,
    SplToken,
}

impl fmt::Display for TokenStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenStandard::ClassicSplToken => write!(f, "Classic SPL Token"),
            TokenStandard::SplToken2022 => write!(f, "SPL Token-2022"),
            TokenStandard::Native => write!(f, "Native"),
            TokenStandard::SplToken => write!(f, "SPL Token"),
        }
    }
}
