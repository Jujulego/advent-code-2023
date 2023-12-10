use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use na::{point, Point2, vector};
use py::{BBox, Holds};
use num_traits::FromPrimitive;
use crate::pipe::{DOWN, LEFT, Pipe, RIGHT, UP};

#[macro_use]
extern crate num_derive;
extern crate nalgebra as na;
extern crate pythagore as py;

mod pipe;

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
    // Load map
    let mut map: Vec<Vec<Pipe>> = Vec::new();
    let mut start: Option<Point2<i16>> = None;

    for (y, line) in read_lines!("day-10/input.txt").enumerate() {
        let mut row = Vec::new();
        let y = y as i16;

        for (x, char) in line.char_indices() {
            let x = x as i16;

            row.push(match char {
                '.' => Pipe::None,
                '-' => Pipe::Horizontal,
                '|' => Pipe::Vertical,
                'F' => Pipe::DownRight,
                '7' => Pipe::DownLeft,
                'J' => Pipe::UpLeft,
                'L' => Pipe::UpRight,
                'S' => {
                    start = Some(point![x, y]);
                    Pipe::None
                }
                _ => panic!("Unknown symbol {char}"),
            });
        }

        map.push(row);
    }

    // Compute start
    let start = start.unwrap();
    let bbox = BBox::from_points(&point![0, 0], &point![map[0].len() as i16, map.len() as i16]);

    let pipe = [(vector![-1, 0], RIGHT), (vector![0, -1], DOWN), (vector![1, 0], LEFT), (vector![0, 1], UP)].iter()
        .map(|(mvt, dir)| (start + mvt, dir))
        .filter(|(pos, &dir)| bbox.holds(pos) && (map[pos.y as usize][pos.x as usize] as u8 & dir == dir))
        .map(|(_, dir)| dir >> 2 | ((dir << 2) % 16))
        .reduce(|acc, d| acc | d).unwrap();

    map[start.y as usize][start.x as usize] = Pipe::from_u8(pipe).unwrap();

    // Move in pipes
    let mut values = HashMap::new();
    let mut queue = VecDeque::new();
    let mut farthest = 0;

    values.insert(start, 0);
    queue.push_front(start);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let pipe = map[pos.y as usize][pos.x as usize];
        let dist = *values.get(&pos).unwrap();

        let neighbors: Vec<Point2<i16>> = [(vector![-1, 0], LEFT), (vector![0, -1], UP), (vector![1, 0], RIGHT), (vector![0, 1], DOWN)].iter()
            .filter(|(_, dir)| pipe as u8 & dir == *dir)
            .map(|(mvt, _)| pos + mvt)
            .filter(|next| !values.contains_key(next))
            .collect();

        for next in neighbors {
            farthest = max(farthest, dist + 1);
            values.insert(next, dist + 1);
            queue.push_back(next);
        }
    }

    println!("part 1: {farthest}");
}
