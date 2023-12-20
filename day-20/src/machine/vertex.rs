use super::{conjunction::Conjunction, flip_flop::FlipFlop};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pulse {
    High,
    Low,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VertexType {
    FlipFlop(FlipFlop),
    Conjunction(Conjunction),
    Broadcaster,
    Dummy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Vertex {
    pub label: String,
    pub inputs_count: u32,
    pub outputs_count: u32,
    pub vertex_type: VertexType,
    pub last_pulse: Option<Pulse>,
}

impl Vertex {
    pub fn new(input: &str) -> Self {
        let first_char = input.chars().next().unwrap();

        let vertex_type = match first_char {
            '%' => VertexType::FlipFlop(FlipFlop::new()),
            '&' => VertexType::Conjunction(Conjunction::new()),
            _ => VertexType::Broadcaster,
        };

        let name = Vertex::create_label(input);

        let vertex = Vertex {
            label: name,
            inputs_count: 0,
            outputs_count: 0,
            vertex_type,
            last_pulse: None,
        };
        vertex
    }

    pub fn pulse(&mut self, pulse: Pulse, input_id: usize) -> Option<Pulse> {
        let pulse = match self.vertex_type {
            VertexType::FlipFlop(ref mut flip_flop) => flip_flop.pulse(pulse),
            VertexType::Conjunction(ref mut conjunction) => conjunction.pulse(pulse, input_id),
            VertexType::Broadcaster => Some(Pulse::Low),
            VertexType::Dummy => None,
        };
        self.last_pulse = pulse;
        pulse
    }

    pub fn create_label(input: &str) -> String {
        let first_char = input.chars().next().unwrap();
        let name;
        match first_char {
            '%' => {
                name = input
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim()
                    .chars()
                    .skip(1)
                    .collect::<String>();
            }
            '&' => {
                name = input
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .trim()
                    .chars()
                    .skip(1)
                    .collect::<String>();
            }
            _ => {
                name = input.split_whitespace().next().unwrap().trim().to_string();
            }
        };
        name
    }

    pub fn plus_input(&mut self) -> u32 {
        self.inputs_count += 1;
        match self.vertex_type {
            VertexType::Conjunction(ref mut conjunction) => conjunction
                .memory
                .insert(self.inputs_count as usize, Pulse::Low),
            _ => None,
        };
        self.inputs_count
    }
}
