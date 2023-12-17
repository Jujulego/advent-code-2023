use std::rc::Rc;
use nalgebra::Point2;

#[derive(Debug)]
pub enum Tree {
    Root(Point2<i32>),
    Node(Point2<i32>, Rc<Tree>),
}

impl Tree {
    pub fn is_child_of(&self, point: &Point2<i32>) -> bool {
        let mut node = self;

        loop {
            match node {
                Tree::Root(pt) => {
                    break pt == point;
                }
                Tree::Node(pt, parent) => {
                    if pt == point {
                        break true;
                    } else {
                        node = parent;
                    }
                }
            }
        }
    }

    pub fn print(&self) {
        let mut node = self;

        loop {
            match node {
                Tree::Root(pt) => {
                    println!("{pt}");
                    break;
                }
                Tree::Node(pt, parent) => {
                    print!("{pt} < ");
                    node = parent;
                }
            }
        }
    }
}