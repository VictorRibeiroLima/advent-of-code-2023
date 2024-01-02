use crate::machine::{self};

pub fn process(input: &str) -> usize {
    let mut machine = machine::Machine::new(input);
    let to_ns_vertex: Vec<String> = machine
        .edges
        .iter()
        .filter(|e| e.to == "ns")
        .map(|e| e.from.clone())
        .collect();

    for edge in to_ns_vertex {
        machine.add_high_watch(&edge);
    }

    while !machine.high_watch_completed() {
        machine.push_button();
    }

    list_lcm(&machine.highs())
}

fn list_lcm(values: &Vec<usize>) -> usize {
    let mut result = values[0];
    for i in 1..values.len() {
        result = lcm(result, values[i]);
    }
    result
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_process() {
        let input = include_str!("./inputs/input.txt");
        assert_eq!(super::process(input), 229414480926893);
    }
}
