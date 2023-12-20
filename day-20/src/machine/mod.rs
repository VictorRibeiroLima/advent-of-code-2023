use std::{cell::RefCell, collections::HashMap};

use self::vertex::{Pulse, Vertex};

mod conjunction;
mod flip_flop;
mod vertex;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    from_id: u32,
    pub from: String,
    pub to: String,
    to_id: u32,
}

#[derive(Debug, Clone)]
pub struct Machine {
    vertices: HashMap<String, RefCell<Vertex>>,
    pub edges: Vec<Edge>,
    low_pulse_count: u32,
    high_pulse_count: u32,
    pub on: bool,
    press_count: u32,
    high_watch: HashMap<String, Option<u32>>,
}

impl PartialEq for Machine {
    fn eq(&self, other: &Machine) -> bool {
        self.vertices == other.vertices && self.edges == other.edges
    }
}

impl Machine {
    pub fn new(input: &str) -> Machine {
        let mut vertices = HashMap::new();
        for line in input.lines() {
            let line = line.trim();
            let vertex = Vertex::new(line);
            let label = vertex.label.clone();
            let vertex = RefCell::new(vertex);
            vertices.insert(label, vertex);
        }
        let mut graph = Machine {
            vertices,
            edges: Vec::new(),
            low_pulse_count: 0,
            high_pulse_count: 0,
            press_count: 0,
            on: false,
            high_watch: HashMap::new(),
        };
        for line in input.lines() {
            let line = line.trim();
            let from = line.split_whitespace().next().unwrap().trim();
            let from = Vertex::create_label(from);
            let connections = line.split_at(line.find("->").unwrap() + 2).1.trim();
            for connection in connections.split(',') {
                let to = connection.trim().to_string();
                graph.add_edge(from.clone(), to);
            }
        }
        graph
    }

    pub fn add_high_watch(&mut self, label: &str) {
        self.high_watch.insert(label.to_string(), None);
    }

    pub fn high_watch_completed(&self) -> bool {
        for (_, value) in self.high_watch.iter() {
            if value.is_none() {
                return false;
            }
        }
        true
    }

    pub fn highs(&self) -> Vec<usize> {
        let mut highs = Vec::new();
        for (_, value) in self.high_watch.iter() {
            highs.push(value.unwrap() as usize);
        }
        highs
    }

    pub fn push_button(&mut self) {
        self.press_count += 1;
        let mut queue = Vec::new();
        let broadcaster_outs = self.get_output_edges("broadcaster");
        let broadcaster_outs_len = broadcaster_outs.len() as u32;
        for edge in broadcaster_outs {
            let vertex = self.vertices.get(&edge.to).unwrap();
            let mut vertex = vertex.borrow_mut();
            let pulse = vertex.pulse(vertex::Pulse::Low, 0);
            if pulse.is_some() {
                queue.push((vertex.label.clone(), pulse.unwrap()));
            }
        }
        self.low_pulse_count += broadcaster_outs_len + 1;

        let mut next_queue = queue;
        loop {
            let mut temp_queue = Vec::new();
            for (from, pulse) in next_queue.into_iter() {
                let output_edges = self.get_output_edges(&from);
                let mut output_edges_len = output_edges.len() as u32;
                if output_edges.is_empty() {
                    output_edges_len = 1;
                }
                for edge in output_edges {
                    let vertex = self.vertices.get(&edge.to).unwrap();
                    let mut vertex = vertex.borrow_mut();

                    let pulse = vertex.pulse(pulse, edge.to_id as usize);
                    let is_in_high_watch = self.high_watch.contains_key(&vertex.label);
                    let vertex_label = vertex.label.clone();
                    drop(vertex);
                    if is_in_high_watch && pulse == Some(Pulse::High) {
                        let value = self.high_watch.get_mut(&vertex_label).unwrap();
                        *value = Some(self.press_count);
                    }
                    if pulse.is_some() {
                        temp_queue.push((vertex_label, pulse.unwrap()));
                    }
                }
                if pulse == vertex::Pulse::Low {
                    self.low_pulse_count += output_edges_len;
                } else {
                    self.high_pulse_count += output_edges_len;
                }
            }
            if temp_queue.is_empty() {
                break;
            }
            next_queue = temp_queue;
        }
    }

    pub fn force(&self) -> u64 {
        self.high_pulse_count as u64 * self.low_pulse_count as u64
    }

    fn get_output_edges(&self, from: &str) -> Vec<Edge> {
        let mut output_edges = Vec::new();
        for edge in self.edges.iter() {
            if edge.from == from {
                output_edges.push(edge.clone());
            }
        }
        output_edges.sort_by(|a, b| a.from_id.cmp(&b.from_id));
        output_edges
    }

    fn add_edge(&mut self, from: String, to: String) {
        let from_vertex = self.vertices.get(&from).unwrap();
        let mut from_vertex = from_vertex.borrow_mut();
        from_vertex.outputs_count += 1;
        let from_id = from_vertex.outputs_count;
        drop(from_vertex);
        let result = self.vertices.get(&to);
        let to_vertex = match result {
            Some(vertex) => vertex,
            None => {
                let vertex = Vertex {
                    inputs_count: 0,
                    label: to.clone(),
                    outputs_count: 0,
                    vertex_type: vertex::VertexType::Dummy,
                    last_pulse: None,
                };
                let vertex = RefCell::new(vertex);
                self.vertices.insert(to.clone(), vertex);
                self.vertices.get(&to).unwrap()
            }
        };
        let mut to_vertex = to_vertex.borrow_mut();

        let to_id = to_vertex.plus_input();
        let edge = Edge {
            from_id,
            from,
            to,
            to_id,
        };
        self.edges.push(edge);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_new() {
        let input = "broadcaster -> a, b, c
      %a -> b
      %b -> c
      %c -> inv
      &inv -> a";
        let graph = super::Machine::new(input);
        assert_eq!(graph.vertices.len(), 5);
        assert_eq!(graph.edges.len(), 7);
    }

    #[test]
    fn test_push_button() {
        let input = "broadcaster -> a, b, c
      %a -> b
      %b -> c
      %c -> inv
      &inv -> a";
        let mut graph = super::Machine::new(input);
        graph.push_button();
        assert_eq!(graph.low_pulse_count, 8);
        assert_eq!(graph.high_pulse_count, 4);
    }

    #[test]
    fn test_push_button2() {
        let input = "broadcaster -> a
        %a -> inv, con
        &inv -> b
        %b -> con
        &con -> output";
        let mut graph = super::Machine::new(input);
        graph.push_button();
        assert_eq!(graph.low_pulse_count, 4);
        assert_eq!(graph.high_pulse_count, 4);
        graph.push_button();
        assert_eq!(graph.low_pulse_count, 8);
        assert_eq!(graph.high_pulse_count, 6);
        graph.push_button();
        assert_eq!(graph.low_pulse_count, 13);
        assert_eq!(graph.high_pulse_count, 9);
        graph.push_button();
        assert_eq!(graph.low_pulse_count, 17);
        assert_eq!(graph.high_pulse_count, 11);
    }
}
