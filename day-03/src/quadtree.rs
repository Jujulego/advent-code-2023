use std::fmt::Debug;
use std::marker::PhantomData;
use na::Point2;
use py::{Holds, Overlaps};
use crate::quadtree::binary_square::BinarySquare;
use crate::quadtree::global_node::GlobalNode;
use crate::quadtree::iter::Iter;
use crate::quadtree::node::Node;
use crate::quadtree::query::Query;
use crate::quadtree::tree::Tree;

mod binary_square;
mod global_node;
mod iter;
mod node;
mod quarter;
mod query;
mod square_node;
mod tree;

/// Quadtree wrapper
#[derive(Clone, Debug)]
pub struct Quadtree<D: Clone + Eq, N: Node<D>> {
    root: N,
    phantom: PhantomData<D>
}

impl<D: Clone + Eq, N: Node<D>> Quadtree<D, N> {
    #[inline]
    pub fn has(&self, point: &Point2<i32>) -> bool {
        self.root.has(point)
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_, D> {
        Iter::new(&self.root)
    }
}

pub type GlobalQuadtree<D> = Quadtree<D, GlobalNode<D>>;

impl<D: Clone + Eq> Quadtree<D, GlobalNode<D>> {
    pub fn new() -> Quadtree<D, GlobalNode<D>> {
        Quadtree {
            root: GlobalNode::new(),
            phantom: PhantomData::default()
        }
    }

    pub fn query<B: Clone + Holds<Point2<i32>> + Overlaps<BinarySquare>>(&self, bbox: &B) -> Query<B, D> {
        Query::new(bbox, &self.root)
    }

    #[inline]
    pub fn insert(&mut self, point: Point2<i32>, data: D) {
        self.root.insert(Tree::Leaf(point, data), &BinarySquare::wrapping(point));
    }

    #[inline]
    pub fn remove(&mut self, point: &Point2<i32>) {
        self.root.remove(point);
    }
}

// Utils
impl<D: Clone + Eq> Default for Quadtree<D, GlobalNode<D>> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, D: Clone + Eq, N: Node<D>> IntoIterator for &'a Quadtree<D, N> {
    type Item = (&'a Point2<i32>, &'a D);
    type IntoIter = Iter<'a, D>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

// Tests
#[cfg(test)]
mod tests {
    use na::point;
    use crate::quadtree::square_node::SquareNode;
    use super::*;

    #[test]
    fn test_has_point() {
        // Initiate tree
        let mut tree = Quadtree::default();
        tree.insert(point![3, 1], 1);
        tree.insert(point![3, 3], 2);
        tree.insert(point![3, 5], 3);

        // Inserted points
        assert!(tree.has(&point![3, 1]));
        assert!(tree.has(&point![3, 3]));
        assert!(tree.has(&point![3, 5]));

        // Others
        assert!(!tree.has(&point![0, 0]));
        assert!(!tree.has(&point![12, 42]));
    }

    #[test]
    fn test_iterator() {
        // Initiate tree
        let mut tree = Quadtree::default();
        tree.insert(point![3, 1], 1);
        tree.insert(point![3, 3], 2);
        tree.insert(point![3, 5], 3);

        // Inserted points
        let mut iter = tree.iter();

        assert_eq!(iter.next(), Some((&point![3, 5], &1)));
        assert_eq!(iter.next(), Some((&point![3, 3], &2)));
        assert_eq!(iter.next(), Some((&point![3, 1], &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_insert_point() {
        // Initiate tree
        let mut tree = Quadtree::default();

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        tree.insert(point![3, 1], 1);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1], 1)
                ]
            }
        );

        // Create a middle node
        tree.insert(point![1, 3], 2);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3], 2),
                            Tree::Leaf(point![3, 1], 1),
                            Tree::Empty
                        ]
                    }))
                ]
            }
        );

        // Insert in middle node
        tree.insert(point![3, 3], 3);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3], 2),
                            Tree::Leaf(point![3, 1], 1),
                            Tree::Leaf(point![3, 3], 3),
                        ]
                    }))
                ]
            }
        );

        // Move the middle node deeper
        tree.insert(point![3, 5], 4);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 8 },
                        children: [
                            Tree::Node(Box::new(SquareNode {
                                area: BinarySquare { anchor: point![0, 0], size: 4 },
                                children: [
                                    Tree::Empty,
                                    Tree::Leaf(point![1, 3], 2),
                                    Tree::Leaf(point![3, 1], 1),
                                    Tree::Leaf(point![3, 3], 3),
                                ]
                            })),
                            Tree::Leaf(point![3, 5], 4),
                            Tree::Empty,
                            Tree::Empty,
                        ],
                    })),
                ]
            }
        );
    }

    #[test]
    fn test_insert_twice() {
        // Initiate tree
        let mut tree = Quadtree::default();

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [Tree::Empty, Tree::Empty, Tree::Empty, Tree::Empty]
            }
        );

        // Insert a point
        tree.insert(point![3, 1], 1);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1], 1),
                ]
            }
        );

        // Insert again point
        tree.insert(point![3, 1], 2);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1], 2),
                ]
            }
        );
    }

    #[test]
    fn test_remove_point() {
        // Initiate tree
        let mut tree = Quadtree::default();
        tree.insert(point![3, 1], 1);
        tree.insert(point![3, 3], 2);
        tree.insert(point![1, 3], 3);
        tree.insert(point![3, 5], 4);

        // Remove point
        tree.remove(&point![3, 3]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 8 },
                        children: [
                            Tree::Node(Box::new(SquareNode {
                                area: BinarySquare { anchor: point![0, 0], size: 4 },
                                children: [
                                    Tree::Empty,
                                    Tree::Leaf(point![1, 3], 3),
                                    Tree::Leaf(point![3, 1], 1),
                                    Tree::Empty
                                ]
                            })),
                            Tree::Leaf(point![3, 5], 4),
                            Tree::Empty,
                            Tree::Empty,
                        ],
                    })),
                ]
            }
        );

        // Simplify by moving node up
        tree.remove(&point![3, 5]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Node(Box::new(SquareNode {
                        area: BinarySquare { anchor: point![0, 0], size: 4 },
                        children: [
                            Tree::Empty,
                            Tree::Leaf(point![1, 3], 3),
                            Tree::Leaf(point![3, 1], 1),
                            Tree::Empty
                        ]
                    })),
                ]
            }
        );

        // Simplify by moving point up
        tree.remove(&point![1, 3]);

        assert_eq!(
            tree.root,
            GlobalNode {
                children: [
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Empty,
                    Tree::Leaf(point![3, 1], 1),
                ]
            }
        );
    }
}
