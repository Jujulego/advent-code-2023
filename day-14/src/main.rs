extern crate nalgebra as na;
extern crate pythagore as py;

use std::cmp::max;
use std::collections::HashMap;
use na::point;
use crate::quadtree::{GlobalQuadtree, Quadtree};
use crate::rock::Rock;
use crate::rock::Rock::{Round, Square};

mod rock;
mod quadtree;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn north_tilt(rocks: &GlobalQuadtree<Rock>, columns: i32) -> GlobalQuadtree<Rock> {
    let mut result = Quadtree::new();

    for col in 0..columns {
        let mut free_row = 0;

        for (pt, rock) in rocks.column_query(col).rev() {
            match rock {
                Round => {
                    result.insert(point![pt.x, free_row], Round);
                    free_row += 1;
                }
                Square => {
                    result.insert(*pt, Square);
                    free_row = pt.y + 1;
                }
            }
        }
    }

    result
}

fn west_tilt(rocks: &GlobalQuadtree<Rock>, rows: i32) -> GlobalQuadtree<Rock> {
    let mut result = Quadtree::new();

    for row in 0..rows {
        let mut free_col = 0;

        for (pt, rock) in rocks.row_query(row).rev() {
            match rock {
                Round => {
                    result.insert(point![free_col, pt.y], Round);
                    free_col += 1;
                }
                Square => {
                    result.insert(*pt, Square);
                    free_col = pt.x + 1;
                }
            }
        }
    }

    result
}

fn south_tilt(rocks: &GlobalQuadtree<Rock>, columns: i32, rows: i32) -> GlobalQuadtree<Rock> {
    let mut result = Quadtree::new();

    for col in 0..columns {
        let mut free_row = rows - 1;

        for (pt, rock) in rocks.column_query(col) {
            match rock {
                Round => {
                    result.insert(point![pt.x, free_row], Round);
                    free_row -= 1;
                }
                Square => {
                    result.insert(*pt, Square);
                    free_row = pt.y - 1;
                }
            }
        }
    }

    result
}

fn east_tilt(rocks: &GlobalQuadtree<Rock>, columns: i32, rows: i32) -> GlobalQuadtree<Rock> {
    let mut result = Quadtree::new();

    for row in 0..rows {
        let mut free_col = columns - 1;

        for (pt, rock) in rocks.row_query(row) {
            match rock {
                Round => {
                    result.insert(point![free_col, pt.y], Round);
                    free_col -= 1;
                }
                Square => {
                    result.insert(*pt, Square);
                    free_col = pt.x - 1;
                }
            }
        }
    }

    result
}

fn print_rocks(rocks: &GlobalQuadtree<Rock>, columns: i32, rows: i32) {
    for y in 0..rows {
        for x in 0..columns {
            match rocks.get(&point![x, y]) {
                Some(Round) => print!("O"),
                Some(Square) => print!("#"),
                None => print!("."),
            }
        }

        println!();
    }
}

fn main() {
    let mut rocks = Quadtree::new();
    let mut columns = 0;
    let mut rows = 0;

    for (y, line) in read_lines!("day-14/input.txt").enumerate() {
        let y = y as i32;

        columns = max(columns, line.len() as i32);
        rows += 1;

        for (x, char) in line.char_indices() {
            let x = x as i32;
            
            match char {
                'O' => rocks.insert(point![x, y], Round),
                '#' => rocks.insert(point![x, y], Square),
                _ => {}
            }
        }
    }

    let mut cache: HashMap<GlobalQuadtree<Rock>, GlobalQuadtree<Rock>> = HashMap::new();
    let mut loop_start_idx = 0;
    let mut target_idx = i32::MAX;
    let mut loop_start = None;

    for i in 0..1000000000 {
        if let Some(result) = cache.get(&rocks) {
            rocks = (*result).clone();

            if let Some(loop_start) = &loop_start {
                if result == loop_start {
                    let loop_len = i - loop_start_idx;
                    target_idx = loop_start_idx + loop_len + ((1000000000 - loop_start_idx) % loop_len) - 1;

                    println!("loop_start: {loop_start_idx}");
                    println!("loop_end: {i}");
                    println!("target: {target_idx}");
                }
            } else {
                loop_start = Some(result.clone());
                loop_start_idx = i;
            }
        } else {
            let result = north_tilt(&rocks, columns);
            let result = west_tilt(&result, rows);
            let result = south_tilt(&result, columns, rows);
            let result = east_tilt(&result, columns, rows);

            cache.insert(rocks, result.clone());
            rocks = result;
        }

        if i == target_idx {
            break;
        }
    }

    let north_load: i32 = rocks.iter().filter(|(_, &rock)| rock == Round)
        .map(|(pt, _)| rows - pt.y)
        .sum();

    println!("north load: {north_load}");
}
