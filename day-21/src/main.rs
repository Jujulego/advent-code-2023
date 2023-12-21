use std::cmp::min;
use std::collections::{HashMap, HashSet, VecDeque};
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

fn count_in(block: &Vec<Vec<char>>, start: Point2<usize>, limit: usize) -> usize {
    let mut distances: HashMap<Point2<usize>, usize> = HashMap::new();
    let mut queue: VecDeque<(Point2<usize>, usize)> = VecDeque::new();

    queue.push_back((start, 0));

    while let Some((pt, d)) = queue.pop_front() {
        if distances.contains_key(&pt) {
            continue;
        }

        if block.get(pt.y).and_then(|row| row.get(pt.x)).unwrap_or(&'#') == &'#' {
            continue;
        }

        if d < limit {
            if pt.y > 0 { queue.push_back((point![pt.x, pt.y - 1], d + 1)); }
            if pt.x > 0 { queue.push_back((point![pt.x - 1, pt.y], d + 1)); }
            queue.push_back((point![pt.x, pt.y + 1], d + 1));
            queue.push_back((point![pt.x + 1, pt.y], d + 1));
        }

        distances.insert(pt, d);
    }

    distances.values().filter(|&d| (d <= &limit) && (d % 2) == 0).count()
}

fn count_in_block(block: Point2<i64>, start: Point2<usize>, map: &Vec<Vec<char>>) -> usize {
    let size_x = map.get(0).map(|row| row.len()).unwrap_or(0);
    let size_y = map.len();

    let origin = point![
        (start.x as i64 - (block.x.signum() * start.x as i64)) as usize,
        (start.y as i64 - (block.y.signum() * start.y as i64)) as usize,
    ];

    let dx = if block.x == 0 { 0 } else { (block.x.abs() as usize - 1) * size_x + start.x + 1 };
    let dy = if block.y == 0 { 0 } else { (block.y.abs() as usize - 1) * size_y + start.y + 1 };

    let limit = min(STEPS - dx - dy, size_x + size_y);
    return count_in(&map, origin, limit);
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
    println!("part 1: {}", count_in(&map, start, 64));

    // Walk in blocks !
    let size_x = map.get(0).map(|row| row.len()).unwrap_or(0);
    let size_y = map.len();

    let mut cache: HashMap<(Point2<usize>, usize), usize> = HashMap::new();
    let mut marks: HashSet<Point2<i64>> = HashSet::new();
    let mut queue: VecDeque<Point2<i64>> = VecDeque::new();
    let mut total = 0;

    queue.push_back(point![0, 0]);

    while let Some(block) = queue.pop_front() {
        if marks.contains(&block) {
            continue;
        }

        marks.insert(block);

        // Walk in block
        let origin = point![
            (start.x as i64 - (block.x.signum() * start.x as i64)) as usize,
            (start.y as i64 - (block.y.signum() * start.y as i64)) as usize,
        ];

        let dx = if block.x == 0 { 0 } else { (block.x.abs() as usize - 1) * size_x + start.x + 1 };
        let dy = if block.y == 0 { 0 } else { (block.y.abs() as usize - 1) * size_y + start.y + 1 };

        //println!("{block} => {dx} + {dy} ({origin})");

        if dx + dy < STEPS {
            let limit = min(STEPS - dx - dy, size_x + size_y);

            if let Some(cnt) = cache.get(&(origin, limit)) {
                total += cnt;
            } else {
                let cnt = count_in(&map, origin, limit);
                total += cnt;

                println!("{origin} + {limit} => {cnt}");
                cache.insert((origin, limit), cnt);
            }

            // Next blocks
            queue.push_back(point![block.x, block.y - 1]);
            queue.push_back(point![block.x - 1, block.y]);
            queue.push_back(point![block.x, block.y + 1]);
            queue.push_back(point![block.x + 1, block.y]);
        }
    }

    println!("part 2: {total}");
}
