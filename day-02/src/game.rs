use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug)]
pub struct Game {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl FromStr for Game {
    type Err = std::num::ParseIntError;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let mut result = Game {
            red: 0,
            green: 0,
            blue: 0,
        };

        for count_color in str.split(',') {
            let (count, color) = count_color.trim().split(' ').collect_tuple().unwrap();

            match color {
                "red" => result.red = u32::from_str_radix(count, 10)?,
                "green" => result.green = u32::from_str_radix(count, 10)?,
                "blue" => result.blue = u32::from_str_radix(count, 10)?,
                _ => {}
            }
        }

        Ok(result)
    }
}

impl Game {
    pub fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}