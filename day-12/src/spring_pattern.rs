use std::fmt::{Debug, Display, Formatter};
use std::ops::{Index, IndexMut};
use std::str::FromStr;
use crate::spring_state::SpringState;
use crate::spring_state::SpringState::{Damaged, Operational, Unknown};

#[derive(Clone, Debug)]
pub struct SpringPattern {
    states: Vec<SpringState>,
}

impl SpringPattern {
    pub fn new(states: Vec<SpringState>) -> Self {
        SpringPattern { states }
    }

    pub fn empty() -> Self {
        SpringPattern::new(Vec::new())
    }

    pub fn ends_with(&self, state: SpringState) -> bool {
        self.states.ends_with(&[state])
    }

    pub fn push(&mut self, state: SpringState) {
        self.states.push(state);
    }

    pub fn unfold(&self, n: usize) -> Self {
        let mut states = self.states.clone();
        states.reserve((n - 1) * (self.states.len() + 1));

        for _ in 1..n {
            states.push(Unknown);
            self.states.iter().for_each(|s| states.push(*s));
        }

        SpringPattern::new(states)
    }

    pub fn start_matches(&self, other: &Self) -> bool {
        self.states.iter()
            .zip(other.states.iter())
            .all(|pair| match pair {
                (Unknown, Unknown) | (Unknown, _) | (_, Unknown) => true,
                (a, b) => a == b,
            })
    }

    pub fn is_empty(&self) -> bool {
        self.states.is_empty()
    }

    pub fn len(&self) -> usize {
        self.states.len()
    }
}

impl Index<usize> for SpringPattern {
    type Output = SpringState;

    fn index(&self, index: usize) -> &Self::Output {
        &self.states[index]
    }
}

impl IndexMut<usize> for SpringPattern {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.states[index]
    }
}

#[derive(Debug)]
pub struct InvalidStateErr {
    pub state: char,
}

impl FromStr for SpringPattern {
    type Err = InvalidStateErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(SpringPattern::new(s.chars()
            .map(|char| match char {
                '#' => Ok(Damaged),
                '.' => Ok(Operational),
                '?' => Ok(Unknown),
                _ => Err(InvalidStateErr { state: char }),
            })
            .collect::<Result<Vec<_>, _>>()?
        ))
    }
}

impl Display for SpringPattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.states.iter()
            .map(|s| <SpringState as Display>::fmt(s, f))
            .collect()
    }
}