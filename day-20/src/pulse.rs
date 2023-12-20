use std::fmt::{Display, Formatter};

// Data
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Pulse {
    Low,
    High
}

// Implementations
impl Display for Pulse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Pulse::Low => write!(f, "low"),
            Pulse::High => write!(f, "high"),
        }
    }
}