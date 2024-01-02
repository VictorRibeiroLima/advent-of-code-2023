use std::collections::{HashMap, HashSet};

pub fn process(input: &str) -> usize {
    let mut lines = input.lines();

    let first = lines.next().unwrap();
    let indexes = first
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();

    let lines = lines.filter(|line| line.len() > 0).collect::<Vec<&str>>();
    let mut end_nodes: HashSet<usize> = HashSet::new();
    let mut starts: Vec<usize> = Vec::new();

    let mut str_to_index: HashMap<&str, usize> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let (key, _) = line.split_at(line.find("=").unwrap());
        let key = key.trim();
        str_to_index.insert(key, i);
        if key.ends_with("Z") {
            end_nodes.insert(i);
        } else if key.ends_with("A") {
            starts.push(i);
        }
    }

    let mut index_cords: HashMap<usize, (usize, usize)> = HashMap::new();

    let mut results: Vec<usize> = Vec::new();

    for start in starts {
        let mut i = 0;
        let mut result = 0;
        let mut current = start;
        loop {
            if i >= indexes.len() {
                i = 0;
            }
            let (founded, c) = is_on_end_node(
                &lines,
                current,
                &end_nodes,
                &indexes,
                &str_to_index,
                i,
                &mut index_cords,
            );

            current = c;
            i += 1;
            result += 1;
            if founded {
                results.push(result);
                break;
            }
        }
    }

    let lcm = list_lcm(&results);

    return lcm;
}

fn is_on_end_node(
    lines: &Vec<&str>,
    current: usize,
    end_nodes: &HashSet<usize>,
    indexes: &Vec<usize>,
    str_to_index: &HashMap<&str, usize>,
    i: usize,
    mut index_cords: &mut HashMap<usize, (usize, usize)>,
) -> (bool, usize) {
    let line = lines[current];
    let index = indexes[i];

    let cords = match index_cords.get(&current) {
        Some(cords) => *cords,
        None => add_to_map(line, &str_to_index, &mut index_cords, current),
    };

    let current = match index {
        0 => cords.0,
        1 => cords.1,
        _ => panic!("Invalid index"),
    };
    return (end_nodes.contains(&current), current);
}

fn add_to_map(
    line: &str,
    str_to_index: &HashMap<&str, usize>,
    index_cords: &mut HashMap<usize, (usize, usize)>,
    current: usize,
) -> (usize, usize) {
    let (_, value) = line.split_at(line.find("=").unwrap() + 1);
    let value = value.trim();
    let value = value.replace("(", "").replace(")", "");
    let (first, second) = value.split_at(value.find(",").unwrap());
    let first = first.trim();
    let second = second.replace(",", "");
    let second = second.trim();
    let first = str_to_index.get(first).unwrap();
    let second = str_to_index.get(second).unwrap();
    let value = (*first, *second);
    index_cords.insert(current, value);
    value
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
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_2() {
        let input = include_str!("../inputs/test2.txt");
        let result = process(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_3() {
        let input = include_str!("../inputs/test3.txt");
        let result = process(input);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_4() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 11_283_670_395_017);
    }
}
