use std::collections::{HashMap, VecDeque};
use crate::part::Part;
use crate::part_range::PartRange;
use crate::rule::RuleResult::{Accepted, Target, Refused};
use crate::workflow::Workflow;

mod part;
mod rule;
mod workflow;
mod part_range;

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
    let mut lines = read_lines!("day-19/input.txt");

    // Parse workflows
    let mut workflows: HashMap<String, Workflow> = HashMap::new();

    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }

        if let Some(bracket) = line.find('{') {
            workflows.insert(line[..bracket].to_string(), line[bracket..].parse().unwrap());
        }
    }

    // Parse parts
    let start = Target("in".to_string());
    let mut part1 = 0;

    for line in lines {
        let mut result = &start;
        let part: Part = line.parse().unwrap();

        while let Target(workflow) = result {
            result = workflows.get(workflow).unwrap().process(&part);
        }

        println!("{:?} => {:?}", part, result);

        if *result == Accepted {
            part1 += part.x + part.m + part.a + part.s
        }
    }

    println!("part 1: {part1}");

    // Process part range
    let mut queue = VecDeque::new();
    let mut part2 = 0;

    queue.push_back((PartRange::new(), &start));

    while let Some((range, result)) = queue.pop_front() {
        match result {
            Accepted => {
                println!("accepted {:?} => {}", range, range.size());
                part2 += range.size();
            },
            Refused => {},
            Target(workflow) => {
                queue.extend(workflows.get(workflow).unwrap().process_range(range));
            }
        }
    }

    println!("part 2: {part2}");
}
