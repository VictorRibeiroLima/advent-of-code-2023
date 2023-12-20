use std::collections::HashMap;

use super::vertex::Pulse;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conjunction {
    pub memory: HashMap<usize, Pulse>,
}

impl Conjunction {
    pub fn new() -> Conjunction {
        Conjunction {
            memory: HashMap::new(),
        }
    }

    pub fn pulse(&mut self, pulse: Pulse, input_id: usize) -> Option<Pulse> {
        self.memory.insert(input_id, pulse);
        let all_high = self.memory.values().all(|&pulse| pulse == Pulse::High);
        if all_high {
            Some(Pulse::Low)
        } else {
            Some(Pulse::High)
        }
    }
}
