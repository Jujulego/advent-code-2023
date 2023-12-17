use std::collections::{BinaryHeap, HashMap};
use nalgebra::{point, Point2, Vector2};
use step::Step;
use crate::direction::Direction;
use crate::direction::Direction::{Down, Right};

mod step;
mod direction;

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

fn is_strait_line(mut from: Point2<i32>, direction: Direction, previous_map: &HashMap<Point2<i32>, Point2<i32>>) -> bool {
    let mut cnt = 0;

    while let Some(&previous) = previous_map.get(&from) {
        if previous == (from - Vector2::from(direction)) {
            from = previous;
            cnt += 1;

            if cnt > 2 {
                return true
            }
        } else {
            break;
        }
    }

    false
}

fn main() {
    let heat_map: Vec<Vec<u32>> = read_lines!("day-17/input.txt")
        .map(|line| line.chars().map(|c| (c as u32) - ZERO).collect())
        .collect();

    let target = point![(heat_map[0].len() - 1) as i32, (heat_map.len() - 1) as i32];
    let bbox = point![0, 0]..=target;

    // Djikstra
    let mut queue = BinaryHeap::new();
    let mut marks = HashMap::new();
    let mut previous_map = HashMap::new();

    queue.push(Step {
        position: point![0, 0],
        heat_loss: 0,
        moves: [None, None, Some(Right)],
    });

    while let Some(step) = queue.pop() {
        let last = step.moves[2].unwrap_or(Right);

        if step.position == target {
            println!("part 1: {}", step.heat_loss);
            break;
        }

        for direction in [last.left_turn(), last, last.right_turn()] {
            let next = step.position + Vector2::from(direction);

            if step.moves.iter().all(|&m| m == Some(direction)) {
                continue;
            }

            if bbox.contains(&next) {
                let heat_loss = step.heat_loss + heat_map[next.y as usize][next.x as usize];

                if marks.get(&next).unwrap_or(&u32::MAX) < &heat_loss {
                    continue;
                }

                marks.insert(next, heat_loss);
                previous_map.insert(next, step.position);

                queue.push(Step {
                    position: next,
                    heat_loss,
                    moves: [step.moves[1], step.moves[2], Some(direction)]
                });

                println!("{} => {next} ({heat_loss})", step.position);
            }
        }
    }

    for y in 0..=target.y {
        for x in 0..=target.x {
            let pos = point![x, y];

            if let Some(prev) = previous_map.get(&pos) {
                print!("{}", Direction::from(pos - prev));
            } else {
                print!(" ");
            }
        }

        println!();
    }
}
