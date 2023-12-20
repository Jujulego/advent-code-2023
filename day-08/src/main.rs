use std::collections::{HashMap, HashSet};
use num::Integer;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

#[derive(Debug, Eq, PartialEq)]
pub enum Move {
    Right,
    Left,
}

fn count_steps<'a>(mut pos: &'a str, moves: &Vec<Move>, nodes: &'a HashMap<String, (String, String)>) -> (&'a str, usize) {
    let mut cnt = 0;

    while !pos.ends_with('Z') || cnt == 0 {
        let mvt = &moves[cnt % moves.len()];
        let node = nodes.get(pos).unwrap();

        pos = match mvt {
            Move::Left => &node.0,
            Move::Right => &node.1
        };

        cnt += 1;
    }

    (pos, cnt)
}

fn factorize(val: usize) -> Vec<usize> {
    let limit = (val as f64).sqrt().ceil() as usize;
    (2..=limit).into_iter().filter(|f| val % f == 0).flat_map(|f| [f, val / f]).collect()
}

fn main() {
    let mut lines = read_lines!("day-08/input.txt");

    // Read moves
    let moves: Vec<Move> = lines.next().unwrap().chars()
        .map(|c| if c == 'R' { Move::Right } else { Move::Left }).collect();

    // Read nodes
    let mut nodes: HashMap<String, (String, String)> = HashMap::new();

    lines.next();

    for line in lines {
        let node = String::from(&line[0..3]);
        let left = String::from(&line[7..10]);
        let right = String::from(&line[12..15]);

        nodes.insert(node, (left, right));
    }

    // Follow path
    let position: Vec<&String> = nodes.keys().filter(|n| n.ends_with('A')).collect();
    let mut part2: usize = 1;

    for pos in position {
        let (_, cnt) = count_steps(pos, &moves, &nodes);

        if pos == "AAA" {
            println!("part 1: {cnt}");
        }

        part2 = part2.lcm(&cnt);
    }

    println!("part 2: {part2}");
}
