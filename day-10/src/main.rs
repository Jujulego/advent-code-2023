use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use na::{point, Point2, vector, Vector2};
use py::{BBox, Holds};
use num_traits::FromPrimitive;
use crate::pipe::{DOWN, LEFT, Pipe, RIGHT, UP};

#[macro_use]
extern crate num_derive;
extern crate nalgebra as na;
extern crate pythagore as py;

mod pipe;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

const MOVES: [(Vector2<i16>, u8); 4] = [(vector![-1, 0], LEFT), (vector![0, -1], UP), (vector![1, 0], RIGHT), (vector![0, 1], DOWN)];

fn opposite_dir(dir: u8) -> u8 {
    dir >> 2 | ((dir << 2) % 16)
}

fn main() {
    // Load map
    let mut map: Vec<Vec<Pipe>> = Vec::new();
    let mut start: Option<Point2<i16>> = None;

    for (y, line) in read_lines!("day-10/input.txt").enumerate() {
        let mut row = Vec::new();
        let y = y as i16;

        for (x, char) in line.char_indices() {
            let x = x as i16;

            row.push(match char {
                '.' => Pipe::None,
                '-' => Pipe::Horizontal,
                '|' => Pipe::Vertical,
                'F' => Pipe::DownRight,
                '7' => Pipe::DownLeft,
                'J' => Pipe::UpLeft,
                'L' => Pipe::UpRight,
                'S' => {
                    start = Some(point![x, y]);
                    Pipe::None
                }
                _ => panic!("Unknown symbol {char}"),
            });
        }

        map.push(row);
    }

    // Compute start
    let start = start.unwrap();
    let bbox = BBox::from_points(&point![0, 0], &point![map[0].len() as i16, map.len() as i16]);

    let pipe = MOVES.iter()
        .map(|(mvt, dir)| (start + mvt, dir))
        .filter(|(pos, &dir)| bbox.holds(pos) && (map[pos.y as usize][pos.x as usize] as u8 & opposite_dir(dir) != 0))
        .map(|(_, dir)| *dir)
        .reduce(|acc, d| acc | d).unwrap();

    map[start.y as usize][start.x as usize] = Pipe::from_u8(pipe).unwrap();

    // Move in pipes
    let mut pipe_loop = HashMap::new();
    let mut queue = VecDeque::new();
    let mut farthest = 0;

    pipe_loop.insert(start, 0);
    queue.push_front(start);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        let pipe = map[pos.y as usize][pos.x as usize];
        let dist = *pipe_loop.get(&pos).unwrap();

        let neighbors: Vec<Point2<i16>> = MOVES.iter()
            .filter(|(_, dir)| pipe as u8 & dir == *dir)
            .map(|(mvt, _)| pos + mvt)
            .filter(|next| !pipe_loop.contains_key(next))
            .collect();

        for next in neighbors {
            farthest = max(farthest, dist + 1);
            pipe_loop.insert(next, dist + 1);
            queue.push_back(next);
        }
    }

    println!("part 1: {farthest}");

    // Evaluate in/out
    let mut cnt_in = 0;

    for (y, row) in map.iter().enumerate() {
        let y = y as i16;

        let mut is_in = false;
        let mut pipe_sum = 0;
        print!("\x1b[32;44m");

        for (x, &pipe) in row.iter().enumerate() {
            let x = x as i16;
            let is_pipe = pipe_loop.contains_key(&point![x, y]);

            if is_pipe {
                pipe_sum |= pipe as u8;

                if is_in {
                    print!("{}", match pipe {
                        Pipe::Horizontal => if pipe_sum & UP == UP { "▄" } else { "▀" },
                        Pipe::Vertical => "▌",
                        Pipe::UpRight => "▙",
                        Pipe::DownRight => "▛",
                        _ => "",
                    });
                } else {
                    print!("{}", match pipe {
                        Pipe::Horizontal => if pipe_sum & UP == UP { "▀" } else { "▄" },
                        Pipe::Vertical => "▐",
                        Pipe::UpRight => "▝",
                        Pipe::DownRight => "▗",
                        _ => "",
                    });
                }

                if pipe as u8 & RIGHT == 0 {
                    if (pipe_sum & UP == UP) && (pipe_sum & DOWN == DOWN) {
                        is_in = !is_in;
                    }

                    pipe_sum = 0;
                }

                if is_in {
                    print!("{}", match pipe {
                        Pipe::UpLeft => "▟",
                        Pipe::DownLeft => "▜",
                        _ => "",
                    });
                } else {
                    print!("{}", match pipe {
                        Pipe::UpLeft => "▘",
                        Pipe::DownLeft => "▖",
                        _ => "",
                    });
                }
            } else {
                if is_in {
                    cnt_in += 1;
                }

                print!("{}", if is_in { "█" } else { " " });
            }
        }

        println!("\x1b[m");
    }

    println!("part 2: {cnt_in}");
}
