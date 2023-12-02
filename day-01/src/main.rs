const DIGITS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn starts_with_digit(str: &str) -> Option<u32> {
    // Simple digit
    if let Ok(d) = str[..1].parse::<u32>() {
        return Some(d);
    }

    // Full letter digit
    if let Some((_, val)) = DIGITS.iter().find(|(txt, _)| str.starts_with(txt)) {
        return Some(*val)
    }

    None
}

fn ends_with_digit(str: &str) -> Option<u32> {
    // Simple digit
    if let Ok(d) = str[str.len() - 1..].parse::<u32>() {
        return Some(d);
    }

    // Full letter digit
    if let Some((_, val)) = DIGITS.iter().find(|(txt, _)| str.ends_with(txt)) {
        return Some(*val)
    }

    None
}

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
    let mut sum = 0;

    for line in read_lines!("day-01/input.txt") {
        let first = (0..line.len())
            .find_map(|idx| starts_with_digit(&line[idx..])).unwrap();

        let last = (1..=line.len()).rev()
            .find_map(|idx| ends_with_digit(&line[..idx])).unwrap();

        sum += first * 10 + last;
    }

    println!("{sum}");
}
