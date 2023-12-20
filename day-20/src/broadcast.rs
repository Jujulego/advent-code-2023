use crate::pulse::Pulse;
use crate::pulse_processor::PulseProcessor;
use crate::pulse_targets::PulseTargets;

// Data
#[derive(Debug)]
pub struct Broadcast {
    targets: Vec<String>,
}

// Implementations
impl Broadcast {
    pub fn new() -> Broadcast {
        Broadcast {
            targets: Vec::new()
        }
    }
}

impl PulseProcessor for &Broadcast {
    fn process(self, _origin: &str, pulse: Pulse) -> Option<Pulse> {
        Some(pulse)
    }
}

impl PulseTargets for Broadcast {
    fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn register_target(&mut self, target: &str) -> () {
        self.targets.push(target.to_string());
    }
}