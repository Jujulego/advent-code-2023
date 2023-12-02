mod game;

use std::cmp::max;
use itertools::Itertools;
use crate::game::Game;

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

    for line in read_lines!("day-02/input.txt") {
        let (_, games) = line.split(':').collect_tuple().unwrap();

        let mut minimal = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        let games = games.split(';')
            .map(|game| game.parse::<Game>().unwrap());

        for game in games {
            minimal.red = max(minimal.red, game.red);
            minimal.green = max(minimal.green, game.green);
            minimal.blue = max(minimal.blue, game.blue);
        }

        sum = sum + (minimal.red * minimal.green * minimal.blue);
    }

    println!("{sum}");
}
