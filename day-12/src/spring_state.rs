use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SpringState {
    Damaged,
    Operational,
    Unknown
}

impl Display for SpringState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SpringState::Damaged => write!(f, "#"),
            SpringState::Operational => write!(f, "."),
            SpringState::Unknown => write!(f, "\x1b[37m?\x1b[m"),
        }
    }
}