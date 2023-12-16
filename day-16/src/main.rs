extern crate nalgebra as na;

use std::cmp::max;
use std::collections::{HashSet, VecDeque};
use na::{point, Point2, vector};
use crate::Direction::*;

#[derive(Copy, Clone, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Left,
    Down,
    Right
}

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn opposite(dir: &Direction) -> Direction {
    match dir {
        Up => Down,
        Left => Right,
        Down => Up,
        Right => Left
    }
}

fn energized_from(tiles: &Vec<Vec<char>>, start: Point2<i32>, dir: Direction) -> usize {
    let mut marks = HashSet::new();
    let mut energized = HashSet::new();
    let mut stack = VecDeque::new();

    stack.push_back((start, dir));

    while let Some((pos, dir)) = stack.pop_back() {
        let next = match &dir {
            Up => pos + vector![0, -1],
            Left => pos + vector![-1, 0],
            Down => pos + vector![0, 1],
            Right => pos + vector![1, 0],
        };

        if marks.contains(&(next, dir)) {
            continue;
        }

        let tile = tiles.get(next.y as usize)
            .and_then(|row| row.get(next.x as usize));

        match tile {
            Some('/') => {
                marks.insert((next, dir));
                energized.insert(next);

                stack.push_back(match dir {
                    Up => (next, Right),
                    Left => (next, Down),
                    Down => (next, Left),
                    Right => (next, Up)
                });
            },
            Some('\\') => {
                marks.insert((next, dir));
                energized.insert(next);

                stack.push_back(match dir {
                    Up => (next, Left),
                    Left => (next, Up),
                    Down => (next, Right),
                    Right => (next, Down)
                });
            },
            Some('|') => {
                marks.insert((next, dir));
                marks.insert((next, opposite(&dir)));
                energized.insert(next);

                match dir {
                    Up | Down => {
                        stack.push_back((next, dir));
                    }
                    Left | Right => {
                        stack.push_back((next, Up));
                        stack.push_back((next, Down));
                    }
                }
            },
            Some('-') => {
                marks.insert((next, dir));
                marks.insert((next, opposite(&dir)));
                energized.insert(next);

                match dir {
                    Up | Down => {
                        stack.push_back((next, Left));
                        stack.push_back((next, Right));
                    }
                    Left | Right => {
                        stack.push_back((next, dir));
                    }
                }
            },
            Some('.') => {
                energized.insert(next);
                stack.push_back((next, dir));
            }
            _ => {}
        }
    }

    energized.len()
}

fn main() {
    // Load tiles
    let tiles: Vec<Vec<char>> = read_lines!("day-16/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    let mut optimal = 0;
    let max_y = tiles.len() as i32;
    let max_x = tiles[0].len() as i32;

    for r in 0..max_y {
        let cnt = energized_from(&tiles, point![-1, r], Right);

        if r == 0 {
            println!("part 1: {cnt}");
        }

        optimal = max(optimal, cnt);
        optimal = max(optimal, energized_from(&tiles, point![max_x, r], Left));
    }

    for c in 0..max_x {
        optimal = max(optimal, energized_from(&tiles, point![c, -1], Down));
        optimal = max(optimal, energized_from(&tiles, point![c, max_y], Up));
    }

    println!("part 2: {optimal}");
}
