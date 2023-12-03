use na::Point2;
use crate::quadtree::square_node::SquareNode;

/// Quadtree itself
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Tree<D: Clone + Eq> {
    Leaf(Point2<i32>, D),
    Node(Box<SquareNode<D>>),
    Empty
}
