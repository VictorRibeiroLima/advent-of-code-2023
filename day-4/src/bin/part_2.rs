use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../inputs/my_input_part_2.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u32 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut result = 0;
    let mut memory = HashMap::new();
    for i in 0..lines.len() {
        let line = lines[i];
        let num_of_matches = if memory.contains_key(&i) {
            *memory.get(&i).unwrap()
        } else {
            process_line(line)
        };
        let num_of_matches = *memory.entry(i).or_insert(num_of_matches);
        result += 1;
        sub_process(&lines, &mut result, i, num_of_matches, &mut memory);
    }
    return result;
}

fn sub_process(
    lines: &Vec<&str>,
    result: &mut u32,
    index: usize,
    num_of_matches: usize,
    memory: &mut HashMap<usize, usize>,
) {
    for i in index + 1..index + num_of_matches + 1 {
        if i >= lines.len() {
            break;
        }
        let line = lines[i];
        let num_of_matches = if memory.contains_key(&i) {
            *memory.get(&i).unwrap()
        } else {
            let num_of_matches = process_line(line);
            *memory.entry(i).or_insert(num_of_matches)
        };
        if num_of_matches != 0 {
            sub_process(lines, result, i, num_of_matches, memory);
        };
        *result += 1;
    }
}

/*Original:
  Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
*/
fn process_line(input: &str) -> usize {
    let header_index = input.find(":").unwrap();

    // Gets the "41 48 83 86 17 | 83 86  6 31 17  9 48 53" part
    let input = input[header_index + 2..].trim();

    let result = process_card(input);

    return result;
}

/*
 The Block is: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
*/
fn process_card(input: &str) -> usize {
    let mut result = 0;
    let (numbers, winning_numbers) = input.split_at(input.find("|").unwrap());
    let numbers = numbers.trim();
    let winning_numbers = winning_numbers[1..].trim();
    let numbers: HashSet<u32> = numbers
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    winning_numbers
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .for_each(|x| {
            if numbers.contains(&x) {
                result += 1;
            }
        });
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../inputs/test_input_part_2.txt");
        assert_eq!(process(input), 30);
    }

    #[test]
    fn my_input() {
        let input = include_str!("../inputs/my_input_part_2.txt");
        let result = process(input);
        assert_eq!(result, 5571760);
    }
}
