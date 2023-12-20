use super::vertex::Pulse;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FlipFlop {
    pub on: bool,
}

impl FlipFlop {
    pub fn new() -> FlipFlop {
        FlipFlop { on: false }
    }

    pub fn pulse(&mut self, pulse: Pulse) -> Option<Pulse> {
        match pulse {
            Pulse::High => None,
            Pulse::Low => {
                self.on = !self.on;
                if self.on {
                    Some(Pulse::High)
                } else {
                    Some(Pulse::Low)
                }
            }
        }
    }
}
