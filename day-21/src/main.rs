use std::collections::{HashMap, VecDeque};
use nalgebra::{point, Point2};

const STEPS: usize = 64;

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
    let map: Vec<Vec<char>> = read_lines!("day-21/input.txt")
        .map(|line| line.chars().collect())
        .collect();

    // Search start
    let mut start = point![0, 0];

    for (y, line) in map.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if char == &'S' {
                start = point![x, y];
            }
        }
    }

    // Walk !
    let mut distances: HashMap<Point2<usize>, usize> = HashMap::new();
    let mut queue: VecDeque<(Point2<usize>, usize)> = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((pt, d)) = queue.pop_front() {
        if d > STEPS {
            break;
        }

        if distances.contains_key(&pt) {
            continue;
        }

        if map.get(pt.y).and_then(|row| row.get(pt.x)).unwrap_or(&'#') == &'#' {
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
