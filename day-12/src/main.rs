use crate::spring_generator::SpringGenerator;
use crate::spring_pattern::SpringPattern;

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

fn main() {
    let mut part1 = 0;
    let mut part2 = 0;

    for line in read_lines!("day-12/input.txt") {
        let mut parts = line.split(' ');

        let pattern: SpringPattern = parts.next().unwrap().parse().unwrap();
        let groups: Vec<u16> = parts.next().unwrap()
            .split(',').map(|n| n.parse().unwrap())
            .collect();

        println!("{pattern} => {:?}", groups);

        part1 += SpringGenerator::new(pattern.len(), groups.clone())
            .filter(|result| pattern.matches(result))
            .count();

        // Unfold
        let pattern = pattern.unfold(5);
        let groups = groups.repeat(5);

        println!("{pattern} => {:?}", groups);

        // part2 += SpringGenerator::new(pattern.len(), groups.clone())
        //     .filter(|result| pattern.matches(result))
        //     .count();

        for result in SpringGenerator::new(pattern.len(), groups) {
            if result.matches(&pattern) {
                println!("\x1b[32m{result}\x1b[m");
            } else {
                // println!("{result}");
            }
        }
    }

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
