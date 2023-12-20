use crate::broadcast::Broadcast;
use crate::conjunction::Conjunction;
use crate::flip_flop::FlipFlop;
use crate::pulse::Pulse;
use crate::pulse_processor::PulseProcessor;
use crate::pulse_targets::PulseTargets;

#[derive(Debug)]
pub enum Module {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcast(Broadcast),
}

impl Module {
    pub fn is_conjunction(&self) -> bool {
        if let Module::Conjunction(_) = self {
            true
        } else {
            false
        }
    }
}

impl PulseProcessor for &mut Module {
    fn process(self, origin: &str, pulse: Pulse) -> Option<Pulse> {
        match self {
            Module::FlipFlop(module) => module.process(origin, pulse),
            Module::Conjunction(module) => module.process(origin, pulse),
            Module::Broadcast(module) => module.process(origin, pulse),
        }
    }
}

impl PulseTargets for Module {
    fn targets(&self) -> &Vec<String> {
        match self {
            Module::FlipFlop(module) => module.targets(),
            Module::Conjunction(module) => module.targets(),
            Module::Broadcast(module) => module.targets(),
        }
    }

    fn register_target(&mut self, target: &str) -> () {
        match self {
            Module::FlipFlop(module) => module.register_target(target),
            Module::Conjunction(module) => module.register_target(target),
            Module::Broadcast(module) => module.register_target(target),
        }
    }
}