use std::str::FromStr;
use nalgebra::{vector, Vector2};
use crate::direction::Direction::*;

pub enum Direction {
    Up,
    Left,
    Down,
    Right,
}

impl From<&Direction> for Vector2<i32> {
    fn from(value: &Direction) -> Self {
        match value {
            Up => vector![0, -1],
            Left => vector![-1, 0],
            Down => vector![0, 1],
            Right => vector![1, 0],
        }
    }
}

pub type InvalidDirectionString = ();

impl FromStr for Direction {
    type Err = InvalidDirectionString;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "U" => Ok(Up),
            "L" => Ok(Left),
            "D" => Ok(Down),
            "R" => Ok(Right),
            _ => Err(())
        }
    }
}

pub type InvalidDirectionVector = ();

impl TryFrom<Vector2<i32>> for Direction {
    type Error = InvalidDirectionVector;

    fn try_from(value: Vector2<i32>) -> Result<Self, Self::Error> {
        if value == vector![0, -1] {
            Ok(Up)
        } else if value == vector![-1, 0] {
            Ok(Left)
        } else if value == vector![0, 1] {
            Ok(Down)
        } else if value == vector![1, 0] {
            Ok(Right)
        } else {
            Err(())
        }
    }
}
