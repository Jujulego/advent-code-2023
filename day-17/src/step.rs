use nalgebra::Point2;
use std::cmp::Ordering;
use crate::direction::Direction;

#[derive(Debug, Eq, PartialEq)]
pub struct Step {
    pub position: Point2<i32>,
    pub heat_loss: u32,
    pub moves: [Option<Direction>; 3],
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
