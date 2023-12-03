use na::Point2;
use py::{Holds, Overlaps};
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::node::Node;
use crate::quadtree::tree::Tree;

pub struct Query<'a, B: Holds<Point2<i32>>, D: Clone + Eq> {
    bbox: B,
    stack: Vec<&'a Tree<D>>,
}

impl<'a, B: Clone + Holds<Point2<i32>> + Overlaps<BinarySquare>, D: Clone + Eq> Query<'a, B, D> {
    #[inline]
    pub fn new<N: Node<D>>(bbox: &B, root: &'a N) -> Query<'a, B, D> {
        Query {
            bbox: bbox.clone(),
            stack: root.children().collect(),
        }
    }
}

impl<'a, B: Holds<Point2<i32>> + Overlaps<BinarySquare>, D: Clone + Eq> Iterator for Query<'a, B, D> {
    type Item = (&'a Point2<i32>, &'a D);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.stack.pop() {
                None => return None,
                Some(Tree::Empty) => (),
                Some(Tree::Leaf(pt, data)) => {
                    if self.bbox.holds(pt) {
                        return Some((pt, data));
                    }
                },
                Some(Tree::Node(child)) => {
                    if self.bbox.overlaps(&child.area) {
                        self.stack.extend(&child.children)
                    }
                },
            }
        }
    }
}
