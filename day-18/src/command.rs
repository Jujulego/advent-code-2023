use std::str::FromStr;
use nalgebra::Vector2;
use crate::direction::Direction;

pub struct Command {
    pub direction: Direction,
    pub count: i32,
}

impl Command {
    pub fn vector(&self) -> Vector2<i32> {
        Vector2::from(&self.direction) * self.count
    }
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_ascii_whitespace();

        Ok(Command {
            direction: parts.next().unwrap().parse()?,
            count: parts.next().unwrap().parse().map_err(|_| ())?,
        })
    }
}