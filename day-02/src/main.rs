mod input;
mod game;

use itertools::Itertools;
use input::INPUT;
use crate::game::Game;

fn main() {
    let mut sum = 0;

    for line in INPUT {
        let (id, games) = line.split(':').collect_tuple().unwrap();
        let id = dbg!(id[5..].parse::<u32>().unwrap());

        let all_valid = games.split(';')
            .map(|game| game.parse::<Game>().unwrap())
            .all(|game| game.is_valid());

        if all_valid {
            sum = sum + id;
        }
    }

    println!("{sum}");
}
