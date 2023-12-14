use std::iter::FusedIterator;
use na::Point2;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

pub struct Iter<'a, D: Clone + Eq> {
    stack: Vec<&'a Tree<D>>,
}

impl<'a, D: Clone + Eq> Iter<'a, D> {
    pub fn new<N: Node<D>>(root: &'a N) -> Iter<'a, D> {
        Iter {
            stack: root.children().collect()
        }
    }
}

impl<'a, D: Clone + Eq> Iterator for Iter<'a, D> {
    type Item = (&'a Point2<i32>, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(Tree::Empty) => (),
                Some(Tree::Leaf(pt, data)) => return Some((pt, data)),
                Some(Tree::Node(child)) => self.stack.extend(&child.children),
            }
        }
    }
}

impl<'a, D: Clone + Eq> FusedIterator for Iter<'a, D> {}
