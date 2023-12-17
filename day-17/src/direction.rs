use std::fmt::{Display, Formatter};
use nalgebra::{vector, Vector2};
use crate::direction::Direction::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
pub enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl Direction {
    pub fn left_turn(&self) -> Direction {
        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up
        }
    }

    pub fn right_turn(&self) -> Direction {
        match self {
            Up => Right,
            Left => Up,
            Down => Left,
            Right => Down
        }
    }
}

impl From<Direction> for Vector2<i32> {
    fn from(value: Direction) -> Self {
        match value {
            Up => vector![0, -1],
            Left => vector![-1, 0],
            Down => vector![0, 1],
            Right => vector![1, 0],
        }
    }
}

impl From<Vector2<i32>> for Direction {
    fn from(value: Vector2<i32>) -> Self {
        if value == vector![0, -1] {
            Up
        } else if value == vector![-1, 0] {
            Left
        } else if value == vector![0, 1] {
            Down
        } else if value == vector![1, 0] {
            Right
        } else {
            panic!("Invalid direction vector");
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Up => '^',
            Left => '<',
            Down => 'v',
            Right => '>',
        })
    }
}