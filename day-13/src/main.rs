use crate::land::Land;

mod tile;
mod land;

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
    let mut lines = read_lines!("day-13/input.txt");
    let mut part1 = 0;
    let mut part2 = 0;

    loop {
        // Load land
        let mut land = Land::new();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            land.add_row(line);
        }

        if land.is_empty() {
            break;
        }

        // Compare cols
        for c_idx in 0..land.width() - 1 {
            if land.has_col_mirror(c_idx) {
                part1 += c_idx + 1;
                println!("col mirror after {} !", c_idx + 1);

                break;
            }
        }

        for c_idx in 0..land.width() - 1 {
            if land.has_col_smugged_mirror(c_idx) {
                part2 += c_idx + 1;
                println!("smugged col mirror after {} !", c_idx + 1);

                break;
            }
        }

        // Compare rows
        for r_idx in 0..land.height() - 1 {
            if land.has_row_mirror(r_idx) {
                part1 += (r_idx + 1) * 100;
                println!("row mirror after {} !", r_idx + 1);

                break;
            }
        }

        for r_idx in 0..land.height() - 1 {
            if land.has_row_smugged_mirror(r_idx) {
                part2 += (r_idx + 1) * 100;
                println!("smugged row mirror after {} !", r_idx + 1);

                break;
            }
        }

        println!();
    }

    println!("part 1: {part1}");
    println!("part 2: {part2}");
}
