use std::collections::HashMap;

pub fn process(input: &str) -> u32 {
    let mut result = 0;
    let mut lines = input.lines();

    let first = lines.next().unwrap();
    let indexes = first
        .chars()
        .map(|c| if c == 'L' { 0 } else { 1 })
        .collect::<Vec<usize>>();

    let lines = lines.filter(|line| line.len() > 0).collect::<Vec<&str>>();
    let mut target = lines.len() - 1;
    let mut start = 0;

    let mut str_to_index: HashMap<&str, usize> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let (key, _) = line.split_at(line.find("=").unwrap());
        let key = key.trim();
        str_to_index.insert(key, i);
        if key == "ZZZ" {
            target = i;
        }
        if key == "AAA" {
            start = i;
        }
    }

    let mut index_cords: HashMap<usize, (usize, usize)> = HashMap::new();
    let mut current = start;
    let mut i: usize = 0;
    while current != target {
        if i >= indexes.len() {
            i = 0;
        }
        let line = lines[current];
        let index = indexes[i];

        let cords = match index_cords.get(&current) {
            Some(cords) => *cords,
            None => add_to_map(line, &str_to_index, &mut index_cords, current),
        };

        current = match index {
            0 => cords.0,
            1 => cords.1,
            _ => panic!("Invalid index"),
        };
        i += 1;
        result += 1;
    }

    return result;
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
    fn test_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 15871);
    }
}
