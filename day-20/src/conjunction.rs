use std::collections::HashMap;
use crate::pulse::Pulse;
use crate::pulse_processor::PulseProcessor;
use crate::pulse_targets::PulseTargets;

// Data
#[derive(Debug)]
pub struct Conjunction {
    state: HashMap<String, Pulse>,
    targets: Vec<String>,
}

// Implementation
impl Conjunction {
    pub fn new() -> Conjunction {
        Conjunction {
            state: HashMap::new(),
            targets: Vec::new(),
        }
    }

    pub fn register_input(&mut self, input: &str) {
        self.state.insert(input.to_string(), Pulse::Low);
    }
}

impl PulseProcessor for &mut Conjunction {
    fn process(self, origin: &str, pulse: Pulse) -> Option<Pulse> {
        self.state.insert(origin.to_string(), pulse);

        if self.state.values().all(|p| p == &Pulse::High) {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}

impl PulseTargets for Conjunction {
    fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn register_target(&mut self, target: &str) -> () {
        self.targets.push(target.to_string());
    }
}