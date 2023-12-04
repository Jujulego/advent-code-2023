use std::collections::HashMap;
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
    let mut scratchcards: HashMap<u32, u32> = HashMap::new();
    let mut current_card = 1;
    let mut part1 = 0;

    for line in read_lines!("day-04/input.txt") {
        if let Some(cnt) = scratchcards.get_mut(&current_card) {
            *cnt += 1;
        } else {
            scratchcards.insert(current_card, 1);
        }

        let winning = line[10..39].split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.parse().unwrap())
            .collect::<Vec<u32>>();

        let mut score = 0;
        let numbers = line[42..].split(' ')
            .filter(|s| !s.is_empty())
            .map(|n| n.trim_start().parse::<u32>().unwrap());

        let mut win_cnt = 0;
        let current_cnt = *scratchcards.get(&current_card).unwrap();

        for number in numbers {
            if winning.contains(&number) {
                if score == 0 {
                    score = 1;
                } else {
                    score <<= 1;
                }

                win_cnt += 1;

                if let Some(cnt) = scratchcards.get_mut(&(current_card + win_cnt)) {
                    *cnt += current_cnt;
                } else {
                    scratchcards.insert(current_card + win_cnt, current_cnt);
                }
            }
        }

        part1 += score;
        current_card += 1;
    }

    println!("part 1: {part1}");
    println!("part 2: {:?}", scratchcards.values().copied().reduce(|acc, v| acc + v));
}
