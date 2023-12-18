use std::str::FromStr;
use nalgebra::Vector2;
use crate::direction::Direction;
use crate::direction::Direction::*;

pub struct HexCommand {
    pub direction: Direction,
    pub count: i32,
}

impl HexCommand {
    pub fn vector(&self) -> Vector2<i32> {
        Vector2::from(&self.direction) * self.count
    }
}

impl FromStr for HexCommand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hex = s.split_ascii_whitespace().skip(2).next().unwrap();

        Ok(HexCommand {
            direction: match &hex[7..8] {
                "0" => Ok(Right),
                "1" => Ok(Down),
                "2" => Ok(Left),
                "3" => Ok(Up),
                _ => Err(())
            }?,
            count: i32::from_str_radix(&hex[2..7], 16).map_err(|_| ())?,
        })
    }
}