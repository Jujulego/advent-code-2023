use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("day-15/input.txt").expect("error opening file day15.txt");
    let buffer = BufReader::new(file);

    let mut sum = 0;

    for part in BufRead::split(buffer, ',' as u8).map(|p| p.unwrap()) {
        sum += part.iter().fold(0, |hash, &p| ((hash + p as usize) * 17) % 256);
    }

    println!("part 1: {sum}");
}
