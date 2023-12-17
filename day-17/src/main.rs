use std::collections::{BinaryHeap, HashMap};
use std::rc::Rc;
use nalgebra::{point, Point2, Vector2};
use step::Step;
use crate::direction::Direction;
use crate::direction::Direction::{Down, Left, Right, Up};
use crate::tree::Tree;

mod step;
mod direction;
mod tree;

const ZERO: u32 = '0' as u32;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn main() {
    let heat_map: Vec<Vec<u32>> = read_lines!("day-17/input.txt")
        .map(|line| line.chars().map(|c| (c as u32) - ZERO).collect())
        .collect();

    let target = point![(heat_map[0].len() - 1) as i32, (heat_map.len() - 1) as i32];
    let bbox = point![0, 0]..=target;

    // Djikstra
    let mut queue = BinaryHeap::new();
    let mut marks: HashMap<(Point2<i32>, Direction), [u32; 10]> = HashMap::new();

    queue.push(Step {
        position: point![0, 0],
        heat_loss: 0,
        moves: [None, None, None],
        path: Rc::new(Tree::Root(point![0, 0])),
    });
    marks.insert((point![0, 0], Up), [0; 10]);
    marks.insert((point![0, 0], Left), [0; 10]);
    marks.insert((point![0, 0], Down), [0; 10]);
    marks.insert((point![0, 0], Right), [0; 10]);

    while let Some(step) = queue.pop() {
        if step.position == target {
            step.path.print();
            println!("part 1: {}", step.heat_loss);
            break;
        }

        for direction in step.moves[2].map_or([Down, Right], |dir| [dir.left_turn(), dir.right_turn()]) {
            let mut next = step.position;
            let mut heat_loss = step.heat_loss;

            for cnt in 0..10 {
                next += Vector2::from(direction);

                if !bbox.contains(&next) {
                    break;
                }

                heat_loss += heat_map[next.y as usize][next.x as usize];

                if cnt < 3 {
                    continue;
                }

                if let Some(heats) = marks.get_mut(&(next, direction)) {
                    if heats[cnt] <= heat_loss {
                        continue;
                    } else {
                        heats[cnt] = heat_loss
                    }
                } else {
                    let mut heats = [u32::MAX; 10];
                    heats[cnt] = heat_loss;

                    marks.insert((next, direction), heats);
                }

                queue.push(Step {
                    position: next,
                    heat_loss,
                    moves: [step.moves[1], step.moves[2], Some(direction)],
                    path: Rc::new(Tree::Node(next, step.path.clone()))
                });
            }
        }
    }
}
