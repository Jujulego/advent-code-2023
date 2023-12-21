use std::collections::{HashMap, VecDeque};
use nalgebra::{point, Point2};

const STEPS: usize = 26501365;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn wrap(mut v: i64, len: usize) -> usize {
    while v < 0 {
        v += len as i64;
    }

    v as usize % len
}

fn main() {
    // Load map
    let map: Vec<Vec<char>> = read_lines!("day-21/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    // Search start
    let mut start = point![0, 0];

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'S' {
                start = point![x as i64, y as i64];
            }
        }
    }

    // Walk !
    let mut distances: HashMap<Point2<i64>, usize> = HashMap::new();
    let mut queue: VecDeque<(Point2<i64>, usize)> = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((pt, d)) = queue.pop_front() {
        if d > STEPS {
            break;
        }

        if distances.contains_key(&pt) {
            continue;
        }

        if map.get(wrap(pt.y, map.len()))
            .and_then(|row| row.get(wrap(pt.x, row.len())))
            .unwrap_or(&'#') == &'#' {
            continue;
        }

        if d < STEPS {
            queue.push_back((point![pt.x, pt.y - 1], d + 1));
            queue.push_back((point![pt.x - 1, pt.y], d + 1));
            queue.push_back((point![pt.x, pt.y + 1], d + 1));
            queue.push_back((point![pt.x + 1, pt.y], d + 1));
        }

        distances.insert(pt, d);
    }

    println!("part 1: {}", distances.values().filter(|&d| (d % 2) == 0).count());
}
