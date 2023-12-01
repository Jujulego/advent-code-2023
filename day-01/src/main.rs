mod input;

use input::INPUT;

fn main() {
    let mut sum = 0;

    for line in INPUT {
        let mut first = 0;

        for char in line.chars() {
            if char.is_digit(10) {
                first = char.to_digit(10).unwrap();
                break;
            }
        }

        let mut last = 0;

        for char in line.chars().rev() {
            if char.is_digit(10) {
                last = char.to_digit(10).unwrap();
                break;
            }
        }

        sum += first * 10 + last;
    }

    println!("{sum}");
}
