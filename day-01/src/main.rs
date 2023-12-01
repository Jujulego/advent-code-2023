mod input;

use input::INPUT;

fn first_digit(line: &str) -> u32 {
    let mut i = 0;

    while i < line.len() {
        match line[i..i + 1].parse::<u32>() {
            Ok(d) => return d,
            _ => {
                let slice = &line[i..];

                if slice.starts_with("one") {
                    return 1;
                } else if slice.starts_with("two") {
                    return 2;
                } else if slice.starts_with("three") {
                    return 3;
                } else if slice.starts_with("four") {
                    return 4;
                } else if slice.starts_with("five") {
                    return 5;
                } else if slice.starts_with("six") {
                    return 6;
                } else if slice.starts_with("seven") {
                    return 7;
                } else if slice.starts_with("eight") {
                    return 8;
                } else if slice.starts_with("nine") {
                    return 9;
                }
            }
        }

        i = i + 1;
    }

    return 0;
}

fn last_digit(line: &str) -> u32 {
    let mut i = line.len();

    while i > 0 {
        match line[i - 1..i].parse::<u32>() {
            Ok(d) => return d,
            _ => {
                let slice = &line[..i];

                if slice.ends_with("one") {
                    return 1;
                } else if slice.ends_with("two") {
                    return 2;
                } else if slice.ends_with("three") {
                    return 3;
                } else if slice.ends_with("four") {
                    return 4;
                } else if slice.ends_with("five") {
                    return 5;
                } else if slice.ends_with("six") {
                    return 6;
                } else if slice.ends_with("seven") {
                    return 7;
                } else if slice.ends_with("eight") {
                    return 8;
                } else if slice.ends_with("nine") {
                    return 9;
                }
            }
        }

        i = i - 1;
    }

    return 0;
}

fn main() {
    let mut sum = 0;

    for line in INPUT {
        let first = first_digit(line);
        let last = last_digit(line);

        sum += dbg!(first * 10 + last);
    }

    println!("{sum}");
}
