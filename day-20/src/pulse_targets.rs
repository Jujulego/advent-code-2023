pub trait PulseTargets {
    fn targets(&self) -> &Vec<String>;
    fn register_target(&mut self, target: &str) -> ();
}