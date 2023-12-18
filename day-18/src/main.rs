use std::collections::{BTreeSet};
use itertools::Itertools;
use nalgebra::{point, Point2};
use crate::command::Command;
use crate::hex_command::HexCommand;

mod command;
mod direction;
mod hex_command;

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
    // Load polygon
    let polygon: Vec<Point2<i32>> = read_lines!("day-18/input.txt")
        .map(|line| line.parse::<HexCommand>().unwrap())
        .scan(point![0, 0], |pt, cmd| {
            *pt += cmd.vector();
            Some(*pt)
        })
        .collect();

    let sorted = {
        let mut tmp = polygon.clone();
        tmp.sort_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)));
        tmp
    };

    println!("{:?}", sorted);

    let mut previous_row = sorted[0].y;
    let mut columns = BTreeSet::new();
    let mut idx = 0;

    let mut area: i64 = 0;

    while idx < sorted.len() {
        let row = sorted[idx].y;

        // Compute area for known columns
        let dy = (row - previous_row - 1) as i64;
        previous_row = row;

        for pair in &columns.iter().chunks(2) {
            let (a, b) = pair.collect_tuple().unwrap();
            println!("+ {} x {dy}", b - a + 1);

            area += (b - a + 1) as i64 * dy;
        }

        // Update columns
        let mut union: Vec<_> = columns.iter().map(|&c| (c, true)).collect();

        while idx < sorted.len() && sorted[idx].y == row {
            let pt = &sorted[idx];
            idx += 1;

            if columns.contains(&pt.x) {
                columns.remove(&pt.x);
            } else {
                columns.insert(pt.x);
            }
        }

        union.extend(columns.iter().map(|&c| (c, false)));
        union.sort_by_key(|(c, _)| *c);

        // Count union of columns & previous
        let mut from = 0;
        let mut was_in = false;
        let mut is_in = false;

        for (c, was) in union {
            if !was_in && !is_in {
                from = c;
            }

            if was {
                was_in = !was_in;
            } else {
                is_in = !is_in;
            }

            if !was_in && !is_in {
                println!("+ {}", c - from + 1);
                area += (c - from + 1) as i64;
            }
        }
    }

    println!("area: {area}")
}
