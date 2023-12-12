use crate::spring_generator::SpringGenerator;
use crate::spring_pattern::SpringPattern;
use crate::spring_state::SpringState::{*};

mod spring_pattern;
mod spring_state;
mod spring_generator;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn simplify(mut pattern: SpringPattern, groups: &Vec<u16>) -> SpringPattern {
    let mut idx = 0;
    let mut group_idx = 0;

    while idx < pattern.len() {
        match pattern[idx] {
            Damaged => {
                let start = idx;
                let group_size = groups[group_idx] as usize;

                while idx < start + group_size {
                    pattern[idx] = Damaged;
                    idx += 1;
                }

                if idx < pattern.len() {
                    pattern[idx] = Operational;
                }

                group_idx += 1;
            }
            Operational => {}
            Unknown => break
        }

        idx += 1;
    }

    pattern
}

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in read_lines!("day-12/input.txt") {
        let mut parts = line.split(' ');

        let pattern: SpringPattern = parts.next().unwrap().parse().unwrap();
        let groups: Vec<u16> = parts.next().unwrap()
            .split(',').map(|n| n.parse().unwrap())
            .collect();

        let simplified = simplify(pattern.clone(), &groups);

        part1 += SpringGenerator::new(&simplified, groups.clone())
            .count();

        // Unfold
        let groups = groups.repeat(5);
        let pattern = simplify(pattern.unfold(5), &groups);

        part2 += SpringGenerator::new(&pattern, groups.clone())
            .count();
    }

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
