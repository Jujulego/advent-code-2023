use std::collections::HashMap;
use crate::part::Part;
use crate::rule::RuleResult::{Accepted, Target};
use crate::workflow::Workflow;

mod part;
mod rule;
mod workflow;

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
}
