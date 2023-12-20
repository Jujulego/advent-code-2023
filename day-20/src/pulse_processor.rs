use crate::pulse::Pulse;

pub trait PulseProcessor {
    /**
     * Process pulse and return sent pulse if any
     */
    fn process(self, origin: &str, pulse: Pulse) -> Option<Pulse>;
}