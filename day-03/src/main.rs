extern crate nalgebra as na;
extern crate pythagore as py;

mod quadtree;
mod number;

use std::collections::HashMap;
use na::{point, Point2};
use crate::number::Number;
use crate::quadtree::Quadtree;

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
    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols = Quadtree::new();
    let mut y = 0;

    for line in read_lines!("day-03/input.txt") {
        let mut chars = line.char_indices().filter(|(_, c)| c != &'.');
        let mut number: Option<Number> = None;

        while let Some((x, c)) = chars.next() {
            let x = x as i32;

            if let Some(d) = c.to_digit(10) {
                match &mut number {
                    Some(n) => {
                        if x == n.position.x + n.size {
                            n.value = n.value * 10 + d;
                            n.size += 1;
                        } else {
                            numbers.push(*n);
                            number = Some(Number {
                                value: d,
                                position: point![x, y],
                                size: 1
                            })
                        }
                    }
                    None => {
                        number = Some(Number {
                            value: d,
                            position: point![x, y],
                            size: 1
                        })
                    },
                }
            } else {
                symbols.insert(point![x, y], c)
            }
        }

        if let Some(n) = number {
            numbers.push(n);
        }

        y += 1;
    }

    let mut gears: HashMap<Point2<i32>, Vec<u32>> = HashMap::new();
    let mut part1 = 0;

    for n in numbers {
        let mut once = false;

        for (pos, symbol) in symbols.query(&n.surroundings()) {
            if !once {
                once = true;
                part1 += n.value;
            }

            if symbol == &'*' {
                if let Some(item) = gears.get_mut(pos) {
                    item.push(n.value);
                } else {
                    gears.insert(*pos, vec![n.value]);
                }
            }
        }
    }

    let mut part2 = 0;

    for vals in gears.values().filter(|v| v.len() > 1) {
        part2 += vals.iter().copied().reduce(|acc, v| acc * v).unwrap();
    }

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
