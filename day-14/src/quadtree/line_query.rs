use std::collections::VecDeque;
use na::Point2;
use py::traits::DimensionBounds;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

pub struct LineQuery<'a, D: Clone + Eq> {
    order: usize,
    value: i32,
    stack: VecDeque<&'a Tree<D>>,
}

impl<'a, D: Clone + Eq> LineQuery<'a, D> {
    #[inline]
    pub fn new<N: Node<D>>(order: usize, value: i32, root: &'a N) -> LineQuery<'a, D> {
        LineQuery {
            order,
            value,
            stack: root.children().collect(),
        }
    }
}

impl<'a, D: Clone + Eq> Iterator for LineQuery<'a, D> {
    type Item = (&'a Point2<i32>, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop_front() {
                None => return None,
                Some(Tree::Empty) => (),
                Some(Tree::Leaf(pt, data)) => {
                    if pt[self.order] == self.value {
                        return Some((pt, data));
                    }
                },
                Some(Tree::Node(child)) => {
                    if child.area.get_bounds(self.order).contains(&self.value) {
                        for node in &child.children {
                            self.stack.push_front(node)
                        }
                    }
                },
            }
        }
    }
}

impl<'a, D: Clone + Eq> DoubleEndedIterator for LineQuery<'a, D> {
    fn next_back(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop_back() {
                None => return None,
                Some(Tree::Empty) => (),
                Some(Tree::Leaf(pt, data)) => {
                    if pt[self.order] == self.value {
                        return Some((pt, data));
                    }
                },
                Some(Tree::Node(child)) => {
                    if child.area.get_bounds(self.order).contains(&self.value) {
                        for node in child.children.iter().rev() {
                            self.stack.push_back(node)
                        }
                    }
                },
            }
        }
    }
}