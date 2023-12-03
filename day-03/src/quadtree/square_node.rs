use std::slice::Iter;
use na::Point2;
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

/// Quadtree node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SquareNode<D: Clone + Eq> {
    pub area: BinarySquare,
    pub children: [Tree<D>; 4],
}

impl<D: Clone + Eq> SquareNode<D> {
    /// Create a new empty node
    #[inline]
    pub fn new(area: BinarySquare) -> SquareNode<D> {
        SquareNode {
            area,
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }
}

impl<D: Clone + Eq> Node<D> for SquareNode<D> {
    #[inline]
    fn children(&self) -> Iter<'_, Tree<D>> {
        self.children.iter()
    }

    #[inline]
    fn child_holding(&self, point: &Point2<i32>) -> &Tree<D> {
        let idx = self.area.quarter(point) as usize;
        unsafe { self.children.get_unchecked(idx) }
    }

    #[inline]
    fn child_holding_mut(&mut self, point: &Point2<i32>) -> &mut Tree<D> {
        let idx = self.area.quarter(point) as usize;
        unsafe { self.children.get_unchecked_mut(idx) }
    }
}
