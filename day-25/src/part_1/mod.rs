use std::io::Write;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vertex {
    label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
//Double edge
struct Edge {
    point_1: String,
    point_2: String,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]

struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    fn new(input: &str) -> Self {
        let mut graph = Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        };
        for line in input.lines() {
            let line_n = line.replace(":", " ");
            for label in line_n.split_whitespace() {
                graph.add_vertex(label.to_string());
            }
        }

        for line in input.lines() {
            let (src, dst) = line.split_once(": ").unwrap();
            let src = src.trim();
            let dst = dst.trim_matches(|c| c == ':');
            for label in dst.split_whitespace() {
                graph.get_vertex_i(src).unwrap();
                graph.get_vertex_i(label).unwrap();
                graph.add_edge(src.to_string(), label.to_string());
            }
        }
        graph
    }

    fn weight(&self) -> usize {
        let mut result = 0;
        for vertex in self.vertices.iter() {
            let mut count = 0;
            for _ in vertex.label.split('_') {
                count += 1;
            }
            if result == 0 {
                result = count;
            } else {
                result *= count;
            }
        }
        result
    }

    fn get_vertex_i(&self, label: &str) -> Option<usize> {
        for (i, vertex) in self.vertices.iter().enumerate() {
            if vertex.label.contains(label) {
                return Some(i);
            }
        }
        None
    }

    #[allow(dead_code)]
    fn get_vertex(&self, label: &str) -> Option<&Vertex> {
        for vertex in self.vertices.iter() {
            if vertex.label.contains(label) {
                return Some(vertex);
            }
        }
        None
    }

    fn get_edge(&self, src: &String, dst: &String) -> Option<usize> {
        for (i, edge) in self.edges.iter().enumerate() {
            if &edge.point_1 == src && &edge.point_2 == dst {
                return Some(i);
            } else if &edge.point_1 == dst && &edge.point_2 == src {
                return Some(i);
            }
        }
        None
    }

    fn get_edges(&self, src: &String, dst: &String) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, edge) in self.edges.iter().enumerate() {
            if src.contains(&edge.point_1) && dst.contains(&edge.point_2) {
                result.push(i);
            } else if src.contains(&edge.point_2) && dst.contains(&edge.point_1) {
                result.push(i);
            }
        }
        result.sort();
        result
    }

    fn add_vertex(&mut self, label: String) -> usize {
        match self.get_vertex_i(&label) {
            Some(i) => i,
            None => {
                let vertex = Vertex { label };
                self.vertices.push(vertex);
                self.vertices.len() - 1
            }
        }
    }

    fn add_edge(&mut self, point_1: String, point_2: String) {
        match self.get_edge(&point_1, &point_2) {
            Some(_) => {}
            None => {
                let edge = Edge { point_1, point_2 };
                self.edges.push(edge);
            }
        }
    }

    fn collapse_vertex(&mut self, edge_i: usize) {
        let edge = self.edges[edge_i].clone();
        self.edges.remove(edge_i);

        let vertex_1_i = self.get_vertex_i(&edge.point_1).unwrap();
        let vertex_2_i = self.get_vertex_i(&edge.point_2).unwrap();

        let vertex_2 = self.vertices[vertex_2_i].clone();
        let vertex_2_label = vertex_2.label;

        let vertex_1 = &mut self.vertices[vertex_1_i];
        let vertex_1_label = vertex_1.label.clone();
        vertex_1
            .label
            .push_str(format!("_{}", vertex_2_label).as_str());

        let edges_to_remove = self.get_edges(&vertex_2_label, &vertex_1_label);
        self.vertices.remove(vertex_2_i);
        for edge_i in edges_to_remove.iter().rev() {
            self.edges.remove(*edge_i);
        }
    }

    fn kager_min_cut(&self) -> Self {
        let mut tries = 10_000;
        while tries > 0 {
            let mut graph = self.clone();
            while graph.vertices.len() > 2 {
                let edge_i = rand::random::<usize>() % graph.edges.len();
                graph.collapse_vertex(edge_i);
            }
            if graph.edges.len() == 3 {
                return graph;
            }
            tries -= 1;
        }

        panic!("No min cut found");
    }

    #[allow(dead_code)]
    fn write_graph_file(&self, filename: &str) {
        let file = std::fs::File::create(filename).unwrap();
        let mut buffer_writer = std::io::BufWriter::new(file);
        buffer_writer
            .write_fmt(format_args!("digraph G {{\n"))
            .unwrap();

        for vertex in self.vertices.iter() {
            let id = format!("N{}", vertex.label);
            buffer_writer
                .write_fmt(format_args!(
                    "    {} [label = \"{}\" shape=box]\n",
                    id, vertex.label
                ))
                .unwrap();
        }

        for edge in self.edges.iter() {
            let vertex = self.get_vertex(&edge.point_1).unwrap();
            let src: String = format!("N{}", vertex.label);

            let vertex = self.get_vertex(&edge.point_2).unwrap();
            let dst = format!("N{}", vertex.label);

            buffer_writer
                .write_fmt(format_args!("    {} -> {} [dir=\"both\"]\n", src, dst))
                .unwrap();
        }

        buffer_writer.write_fmt(format_args!("}}\n")).unwrap();
    }
}

pub fn process(input: &str) -> usize {
    let graph = Graph::new(input);
    let min_cut = graph.kager_min_cut();
    min_cut.weight()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let input = include_str!("../inputs/test.txt");
        let graph = Graph::new(input);
        graph.write_graph_file("./graph/test.dot");
    }

    #[test]
    fn test_new_input() {
        let input = include_str!("../inputs/input.txt");
        let graph = Graph::new(input);
        graph.write_graph_file("./graph/input.dot");
    }

    #[test]
    fn test_collapse_vertex() {
        let input = include_str!("../inputs/test.txt");
        let mut graph = Graph::new(input);
        graph.collapse_vertex(0);
        graph.write_graph_file("./graph/test_collapse.dot");
        graph.collapse_vertex(0);
        graph.write_graph_file("./graph/test_collapse_2.dot");
        graph.collapse_vertex(0);
        graph.write_graph_file("./graph/test_collapse_3.dot");
    }

    #[test]
    fn test_karger_min_cut() {
        let input = include_str!("../inputs/test.txt");
        let graph = Graph::new(input);
        let min_cut = graph.kager_min_cut();
        min_cut.write_graph_file("./graph/test_karger.dot");
    }

    #[test]
    fn test_weigth() {
        let input = include_str!("../inputs/test.txt");
        let graph = Graph::new(input);
        let min_cut = graph.kager_min_cut();
        let weigth = min_cut.weight();
        assert_eq!(weigth, 54);
    }
}
