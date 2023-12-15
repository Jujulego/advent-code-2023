use std::array;
use std::fs::File;
use std::io::{BufRead, BufReader};

const COMMA: u8 = ',' as u8;
const DASH: u8 = '-' as u8;
const EQUAL: u8 = '=' as u8;
const ZERO: u8 = '0' as u8;

#[derive(Debug)]
struct Lens {
    label: Vec<u8>,
    value: u8,
}

fn hash(data: &[u8]) -> usize {
    data.iter().fold(0, |hash, &p| ((hash + p as usize) * 17) % 256)
}

fn main() {
    let file = File::open("day-15/input.txt").expect("error opening file day15.txt");
    let buffer = BufReader::new(file);

    let mut boxes: [Vec<Lens>; 256] = array::from_fn(|_| Vec::new());
    let mut sum = 0;

    for data in buffer.split(COMMA).map(|p| p.unwrap()) {
        sum += hash(&data);

        if data.ends_with(&[DASH]) {
            let label: Vec<u8> = data[..data.len() - 1].iter().copied().collect();
            let box_id = hash(&label);

            if let Some(lens_pos) = boxes[box_id].iter().position(|lens| lens.label == label) {
                boxes[box_id].remove(lens_pos);
            }
        } else if let Some(idx) = data.iter().position(|d| *d == EQUAL) {
            let label: Vec<u8> = data[..idx].iter().copied().collect();
            let box_id = hash(&label);
            let value = data[idx + 1] - ZERO;

            if let Some(lens_pos) = boxes[box_id].iter().position(|lens| lens.label == label) {
                boxes[box_id][lens_pos].value = value;
            } else {
                boxes[box_id].push(Lens { label, value });
            }
        }
    }

    let power: usize = boxes.iter().enumerate()
        .map(|(box_id, lens)| lens.iter().enumerate()
            .map(|(lens_pos, lens)| (box_id + 1) * (lens_pos + 1) * lens.value as usize)
            .sum::<usize>()
        )
        .sum();

    println!("part 1: {sum}");
    println!("part 2: {power}");
}
