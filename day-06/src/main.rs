use std::iter::zip;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn compute(time: u64, record: u64) -> u64 {
    println!("{:?}", (time, record));
    let delta = dbg!(((time * time - 4 * record) as f64).sqrt());

    let x1 = dbg!(((time as f64 - delta) / 2.0).ceil() as u64);
    let x2 = dbg!(((time as f64 + delta) / 2.0).floor() as u64);

    x2 - x1 + 1
}

fn main() {
    // Parse input
    let mut lines = read_lines!("day-06/input.txt");

    let durations = lines.next().unwrap();
    let durations = durations[10..].split(' ').filter(|&s| !s.is_empty()).map(|n| u64::from_str_radix(n, 10).unwrap());

    let distances = lines.next().unwrap();
    let distances = distances[10..].split(' ').filter(|&s| !s.is_empty()).map(|n| u64::from_str_radix(n, 10).unwrap());

    let mut part1 = 1;

    for (time, record) in zip(durations, distances) {
        part1 *= compute(time, record);
    }

    println!("part 1: {part1}");
    println!("part 2: {}", compute(48938466, 261119210191063));
}
