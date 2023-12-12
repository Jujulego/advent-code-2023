use std::str::FromStr;
use crate::state::State;

#[derive(Debug)]
pub struct SpringPattern {
    states: Vec<State>,
}

#[derive(Debug)]
pub struct InvalidStateErr {
    pub state: char,
}

impl FromStr for SpringPattern