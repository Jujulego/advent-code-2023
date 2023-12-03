use std::slice::Iter;
use na::Point2;
use crate::quadtree::node::Node;
use crate::quadtree::quarter::global_quarter;
use crate::quadtree::tree::Tree;

/// Quadtree global node
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GlobalNode<D: Clone + Eq> {
    pub children: [Tree<D>; 4],
}

impl<D: Clone + Eq> GlobalNode<D> {
    /// Create a new empty node
    #[inline]
    pub fn new() -> GlobalNode<D> {
        GlobalNode {
            children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty],
        }
    }
}

// Utils
impl<D: Clone + Eq> Default for GlobalNode<D> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<D: Clone + Eq> Node<D> for GlobalNode<D> {
    #[inline]
    fn children(&self) -> Iter<'_, Tree<D>> {
        self.children.iter()
    }

    #[inline]
    fn child_holding(&self, point: &Point2<i32>) -> &Tree<D> {
        let idx = global_quarter(point) as usize;
        unsafe { self.children.get_unchecked(idx) }
    }

    #[inline]
    fn child_holding_mut(&mut self, point: &Point2<i32>) -> &mut Tree<D> {
        let idx = global_quarter(point) as usize;
        unsafe { self.children.get_unchecked_mut(idx) }
    }
}
