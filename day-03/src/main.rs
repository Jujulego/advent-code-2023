extern crate nalgebra as na;
extern crate pythagore as py;

mod quadtree;
mod number;

use na::point;
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
                symbols.insert(point![x, y])
            }
        }

        if let Some(n) = number {
            numbers.push(n);
        }

        y += 1;
    }

    let mut sum = 0;

    for n in numbers {
        if symbols.query(&n.surroundings()).next().is_some() {
            sum += n.value;
        }
    }

    println!("{sum}");
}
