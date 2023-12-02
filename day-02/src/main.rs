mod input;
mod game;

use std::cmp::max;
use itertools::Itertools;
use input::INPUT;
use crate::game::Game;

fn main() {
    let mut sum = 0;

    for line in INPUT {
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

        dbg!(&minimal);
        sum = sum + (minimal.red * minimal.green * minimal.blue);
    }

    println!("{sum}");
}
