use std::collections::{HashMap, VecDeque};
use crate::broadcast::Broadcast;
use crate::conjunction::Conjunction;
use crate::flip_flop::FlipFlop;
use crate::module::Module;
use crate::pulse::Pulse;
use crate::pulse_processor::PulseProcessor;
use crate::pulse_targets::PulseTargets;

mod broadcast;
mod conjunction;
mod flip_flop;
mod module;
mod pulse;
mod pulse_processor;
mod pulse_targets;

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
    // Parse modules
    let mut modules: HashMap<String, Module> = HashMap::new();
    let mut inverted: HashMap<String, Vec<String>> = HashMap::new();

    for line in read_lines!("day-20/input.txt") {
        if let Some(arrow) = line.find(" -> ") {
            let mut name = line[..arrow].to_string();

            let mut module = match &name[..1] {
                "%" => {
                    name.remove(0);
                    Module::FlipFlop(FlipFlop::new())
                }
                "&" => {
                    name.remove(0);
                    Module::Conjunction(Conjunction::new())
                }
                _ => Module::Broadcast(Broadcast::new()),
            };

            for target in line[(arrow + 4)..].split(", ") {
                module.register_target(target);

                if let Some(inv) = inverted.get_mut(target) {
                    inv.push(name.clone());
                } else {
                    inverted.insert(target.to_string(), Vec::from([name.clone()]));
                }
            }

            modules.insert(name, module);
        }
    }

    // Register conjunction inputs
    let conjunctions = modules.iter_mut()
        .filter_map(|(name, m)| if let Module::Conjunction(c) = m { Some((name, c)) } else { None });

    for (name, module) in conjunctions {
        for origin in inverted.get(name).unwrap() {
            module.register_input(origin)
        }
    }

    // Process pulses
    let mut queue = VecDeque::new();
    let mut index = 0;
    let mut low_cnt = 0;
    let mut high_cnt = 0;

    let mut last_rhythms: HashMap<String, Option<u64>> = HashMap::new();

    for origin in inverted.get(&inverted.get("rx").unwrap()[0]).unwrap() {
        last_rhythms.insert(origin.to_string(), None);
    }

    'main: loop {
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        index += 1;

        while let Some((origin, pulse, target)) = queue.pop_front() {
            //println!("{origin} -{pulse}> {target}");

            if index <= 1000 {
                match pulse {
                    Pulse::Low => low_cnt += 1,
                    Pulse::High => high_cnt += 1,
                }
            }

            if let Some(module) = modules.get_mut(&target) {
                if let Some(result) = module.process(&origin, pulse) {
                    queue.extend(module.targets().iter().map(|next| (target.clone(), result, next.clone())));
                }
            }

            if target == "hf" && pulse == Pulse::High {
                if last_rhythms.get(&origin).unwrap().is_none() {
                    last_rhythms.insert(origin, Some(index));

                    if last_rhythms.values().filter_map(|&o| o).count() == last_rhythms.len() {
                        break 'main;
                    }
                }
            }
        }
    }

    println!("part 1: {}", low_cnt * high_cnt);
    println!("part 2: {:?}", last_rhythms.values().filter_map(|&o| o).reduce(|acc, v| acc * v));
}
