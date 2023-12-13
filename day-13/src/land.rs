use std::cmp::min;
use std::fmt::{Display, Formatter};
use crate::tile::Tile;
use crate::tile::Tile::{Ash, Rock};

#[derive(Clone, Debug)]
pub struct Land {
    tiles: Vec<Vec<Tile>>,
}

impl Land {
    pub fn new() -> Self {
        Land { tiles: Vec::new() }
    }

    pub fn add_row(&mut self, row: String) -> () {
        let mut tiles = Vec::with_capacity(row.len());

        for char in row.chars() {
            match char {
                '.' => tiles.push(Ash),
                '#' => tiles.push(Rock),
                _ => {}
            }
        }

        tiles.shrink_to_fit();
        self.tiles.push(tiles);
    }

    pub fn get_col(&self, idx: usize) -> Vec<&Tile> {
        (0..self.height())
            .map(|r| &self.tiles[r][idx])
            .collect()
    }

    pub fn get_row(&self, idx: usize) -> &Vec<Tile> {
        &self.tiles[idx]
    }

    pub fn has_col_mirror(&self, idx: usize) -> bool {
        let delta_max = min(idx + 1, self.width() - idx - 1);

        (0..delta_max).map(|delta| (idx - delta, idx + delta + 1))
            .all(|(bi, ai)| self.get_col(bi) == self.get_col(ai))
    }

    pub fn has_col_smugged_mirror(&self, idx: usize) -> bool {
        let delta_max = min(idx + 1, self.width() - idx - 1);
        let mut err = 0;

        for delta in 0..delta_max {
            err += self.get_col(idx - delta).into_iter()
                .zip(self.get_col(idx + delta + 1).into_iter())
                .filter(|(a, b)| a != b)
                .count();
        }

        return err == 1;
    }

    pub fn has_row_mirror(&self, idx: usize) -> bool {
        let delta_max = min(idx + 1, self.height() - idx - 1);

        (0..delta_max).map(|delta| (idx - delta, idx + delta + 1))
            .all(|(bi, ai)| self.get_row(bi) == self.get_row(ai))
    }

    pub fn has_row_smugged_mirror(&self, idx: usize) -> bool {
        let delta_max = min(idx + 1, self.height() - idx - 1);
        let mut err = 0;

        for delta in 0..delta_max {
            err += self.get_row(idx - delta).iter()
                .zip(self.get_row(idx + delta + 1).into_iter())
                .filter(|(a, b)| a != b)
                .count();
        }

        return err == 1;
    }

    pub fn height(&self) -> usize {
        self.tiles.len()
    }

    pub fn width(&self) -> usize {
        self.tiles[0].len()
    }

    pub fn is_empty(&self) -> bool {
        self.tiles.is_empty()
    }
}

impl Display for Land {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.tiles {
            for tile in row {
                <Tile as Display>::fmt(tile, f)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}