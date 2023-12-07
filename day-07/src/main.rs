use crate::hand::Hand;

mod card;
mod hand;

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
    let mut hands: Vec<(Hand, u32)> = Vec::new();

    for line in read_lines!("day-07/input.txt") {
        let mut parts = line.split(' ');

        hands.push((
            parts.next().unwrap().parse().unwrap(),
            parts.next().unwrap().parse().unwrap()
        ));
    }

    hands.sort_by_key(|t| t.0);

    let part1 = hands.iter().enumerate()
        .fold(0, |acc, (rank, &(_, bid))| acc + ((rank as u32) + 1) * bid);

    println!("part 1: {part1}");
}
