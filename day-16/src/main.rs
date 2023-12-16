extern crate nalgebra as na;

use std::collections::{HashSet, VecDeque};
use na::{point, vector};
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

fn main() {
    // Load tiles
    let tiles: Vec<Vec<char>> = read_lines!("day-16/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    let mut marks = HashSet::new();
    let mut energized = HashSet::new();
    let mut stack = VecDeque::new();

    stack.push_back((point![-1, 0], Right));

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

    println!("part 1: {}", energized.len());
}
