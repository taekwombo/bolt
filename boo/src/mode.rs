#[derive(PartialEq)]
pub enum Mode {
    Normal,
    Statement,
    Results,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Normal => f.write_str("Normal"),
            Self::Statement => f.write_str("Statement"),
            Self::Results => f.write_str("Results"),
        }
    }
}

