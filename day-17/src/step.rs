use nalgebra::Point2;
use std::cmp::Ordering;
use std::rc::Rc;
use crate::direction::Direction;
use crate::tree::Tree;

#[derive(Debug)]
pub struct Step {
    pub position: Point2<i32>,
    pub heat_loss: u32,
    pub moves: [Option<Direction>; 3],
    pub path: Rc<Tree>,
}

impl Eq for Step {}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heat_loss.cmp(&self.heat_loss)
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.heat_loss == other.heat_loss
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
