
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

    for line in read_lines!("day-04/input.txt") {
        let winning = line[10..39].split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        let mut score = 0;
        let numbers = line[42..].split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.trim_start().parse::<u32>().unwrap());

        for number in numbers {
            if winning.contains(&number) {
                if score == 0 {
                    score = 1;
                } else {
                    score <<= 1;
                }
            }
        }

        part1 += score;
    }

    println!("part 1: {part1}");
}
