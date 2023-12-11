extern crate nalgebra as na;

use std::cmp::max;
use std::collections::HashSet;
use na::{point, Point2};

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn manhattan_dist(a: &Point2<i64>, b: &Point2<i64>) -> u64 {
    b.x.abs_diff(a.x) + b.y.abs_diff(a.y)
}

fn main() {
    // Read galaxies
    let mut galaxies: Vec<Point2<i64>> = Vec::new();

    let mut row_count = 0;
    let mut filled_rows = HashSet::new();

    let mut col_count = 0;
    let mut filled_cols = HashSet::new();

    for (y, line) in read_lines!("day-11/input.txt").enumerate() {
        let y = y as i64;

        for (x, _) in line.char_indices().filter(|(_, c)| *c == '#') {
            let x = x as i64;

            row_count = max(row_count, y);
            col_count = max(col_count, x);

            galaxies.push(point![x, y]);
            filled_rows.insert(y);
            filled_cols.insert(x);
        }
    }

    // Expand universe
    let empty_rows: Vec<i64> = (0..row_count).filter(|r| !filled_rows.contains(r)).collect();
    let empty_cols: Vec<i64> = (0..col_count).filter(|c| !filled_cols.contains(c)).collect();

    for galaxy in &mut galaxies {
        let dy = empty_rows.partition_point(|&r| r < galaxy.y) as i64;
        let dx = empty_cols.partition_point(|&c| c < galaxy.x) as i64;

        galaxy.y += dy * 999999;
        galaxy.x += dx * 999999;
    }

    // Compute distances
    let mut sum = 0;

    for (ia, a) in galaxies.iter().enumerate() {
        for b in &galaxies[(ia + 1)..] {
            sum += manhattan_dist(a, b);
        }
    }

    println!("part 1: {sum}");
}

