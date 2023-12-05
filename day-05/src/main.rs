use std::collections::VecDeque;
use std::ops::Range;
use itertools::Itertools;
use crate::mapping::Mapping;

mod mapping;

macro_rules! read_lines {
    ($file:literal) => {
        {
            let file = std::fs::File::open($file).expect(&format!("error opening file {}", $file));
            let buffer = std::io::BufReader::new(file);
            std::io::BufRead::lines(buffer).map(|line| line.unwrap())
        }
    };
}

fn map_val(map: &Vec<Mapping>, val: u64) -> u64 {
    for mapping in map {
        if let Some(res) = mapping.map(val) {
            return res;
        }
    }

    val
}

fn map_ranges(map: &Vec<Mapping>, ranges: Vec<Range<u64>>) -> Vec<Range<u64>> {
    let mut queue = VecDeque::from(ranges);
    let mut result = Vec::new();

    'queue: while !queue.is_empty() {
        let range = queue.pop_front().unwrap();

        for mapping in map {
            let (mapped, left) = mapping.map_range(&range);

            if let Some(mapped) = mapped {
                result.push(mapped);

                for r in left {
                    if !r.is_empty() {
                        queue.push_front(r);
                    }
                }

                continue 'queue;
            }
        }

        result.push(range);
    }

    result
}

fn main() {
    let lines = read_lines!("day-05/input.txt").collect::<Vec<String>>();

    // Load seeds
    let seeds = lines[0][7..].split(' ')
        .map(|n| u64::from_str_radix(n, 10).unwrap())
        .collect::<Vec<u64>>();

    // Load maps
    let seed_to_soil = lines[3..12].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();
    let soil_to_fertilizer = lines[14..57].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();
    let fertilizer_to_water = lines[59..105].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();
    let water_to_light = lines[107..147].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();
    let light_to_temperature = lines[149..186].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();
    let temperature_to_humidity = lines[188..206].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();
    let humidity_to_location = lines[208..250].iter().map(|l| l.parse().unwrap()).collect::<Vec<Mapping>>();

    let maps = [
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ];

    // Build seed ranges
    let mut ranges = Vec::new();

    for pair in &seeds.iter().chunks(2) {
        let (start, len) = pair.collect_tuple().unwrap();

        ranges.push(*start..(start + len))
    }

    // Search closest location
    let mut closest = u64::MAX;

    for seed in seeds {
        let location = maps.iter().fold(seed, |acc, map| map_val(map, acc));

        if location < closest {
            closest = location;
        }
    }

    println!("part 1: {closest}");

    // Compute location ranges
    let location = maps.iter().fold(ranges, |acc, map| map_ranges(map, acc));
    let closest = location.iter().map(|r| r.start).min().unwrap();

    println!("part 2: {closest}")
}
