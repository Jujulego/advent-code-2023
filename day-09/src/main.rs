

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn compute_factors(numbers: &Vec<i128>) -> Vec<i128> {
    let mut factors = vec![];
    let mut diffs = numbers.clone();

    loop {
        factors.push(diffs[0]);

        diffs = diffs.iter().zip(diffs.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();

        if diffs.iter().all(|n| *n == 0) {
            break;
        }
    }

    factors
}

fn reduced_pow(x: i128, pow: i128) -> i128 {
    (0..pow).map(|k| x - k).product()
}

fn factorielle(n: i128) -> i128 {
    (1..=n).product()
}

fn main() {
    let mut line_num = 0;
    let mut sum_next = 0;
    let mut sum_prev = 0;

    for line in read_lines!("day-09/input.txt") {
        line_num += 1;

        let numbers: Vec<i128> = line.split(' ').map(|n| n.parse().unwrap()).collect();
        let factors = compute_factors(&numbers);

        println!("#{line_num} => factors: {:?}", &factors);

        let x = numbers.len() as i128;

        let res: i128 = factors.iter().enumerate()
            .map(|(i, &f)| f * reduced_pow(x, i as i128) / factorielle(i as i128))
            .sum();

        println!("#{line_num} => next: {res}");

        sum_next += res;

        let res: i128 = factors.iter().enumerate()
            .map(|(i, &f)| f * reduced_pow(-1, i as i128) / factorielle(i as i128))
            .sum();

        println!("#{line_num} => prev: {res}");

        sum_prev += res;
    }

    println!("part 1: {sum_next}");
    println!("part 2: {sum_prev}");
}
