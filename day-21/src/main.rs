use std::collections::{HashMap, VecDeque};
use nalgebra::{point, Point2};

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
        if distances.contains_key(&pt) {
            continue;
        }

        if map.get(pt.y).and_then(|row| row.get(pt.x)).unwrap_or(&'#') == &'#' {
            continue;
        }

        if pt.y > 0 { queue.push_back((point![pt.x, pt.y - 1], d + 1)); }
        if pt.x > 0 { queue.push_back((point![pt.x - 1, pt.y], d + 1)); }
        queue.push_back((point![pt.x, pt.y + 1], d + 1));
        queue.push_back((point![pt.x + 1, pt.y], d + 1));

        distances.insert(pt, d);
    }

    println!("part 1: {}", distances.values().filter(|&d| (*d <= 64) && (d % 2 == 0)).count());

    // Part 2 (cf https://github.com/villuna/aoc23/wiki/A-Geometric-solution-to-advent-of-code-2023,-day-21)
    let even_corners = distances.values().filter(|&d| (*d > start.x) && (d % 2 == 0)).count();
    let odd_corners = distances.values().filter(|&d| (*d > start.x) && (d % 2 == 1)).count();

    let even_full = distances.values().filter(|&d| d % 2 == 0).count();
    let odd_full = distances.values().filter(|&d| d % 2 == 1).count();

    let n = (26501365 - start.y) / map.len();

    print!("part 2: {}", ((n + 1) * (n + 1)) * odd_full + (n * n) * even_full - (n + 1) * odd_corners + n * even_corners);
}
