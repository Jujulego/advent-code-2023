use crate::pulse::Pulse;
use crate::pulse_processor::PulseProcessor;
use crate::pulse_targets::PulseTargets;

// Data
#[derive(Debug)]
enum State {
    On,
    Off
}

#[derive(Debug)]
pub struct FlipFlop {
    state: State,
    targets: Vec<String>,
}

// Implementations
impl FlipFlop {
    pub fn new() -> FlipFlop {
        FlipFlop {
            state: State::Off,
            targets: Vec::new(),
        }
    }
}

impl PulseProcessor for &mut FlipFlop {
    fn process(self, _origin: &str, pulse: Pulse) -> Option<Pulse> {
        match (pulse, &self.state) {
            (Pulse::High, _) => None,
            (Pulse::Low, State::Off) => {
                self.state = State::On;
                Some(Pulse::High)
            }
            (Pulse::Low, State::On) => {
                self.state = State::Off;
                Some(Pulse::Low)
            },
        }
    }
}

impl PulseTargets for FlipFlop {
    fn targets(&self) -> &Vec<String> {
        &self.targets
    }

    fn register_target(&mut self, target: &str) -> () {
        self.targets.push(target.to_string());
    }
}