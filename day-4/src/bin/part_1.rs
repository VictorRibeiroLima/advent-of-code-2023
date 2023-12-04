use std::collections::HashSet;

fn main() {
    let input = include_str!("../inputs/my_input_part_1.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let power = process_line(line);

        result += power;
    }
    return result;
}

/*Original:
  Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
*/
fn process_line(input: &str) -> u32 {
    let header_index = input.find(":").unwrap();

    // Gets the "41 48 83 86 17 | 83 86  6 31 17  9 48 53" part
    let input = input[header_index + 2..].trim();

    let power = process_card(input);

    return power;
}

/*
 The Block is: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
*/
fn process_card(input: &str) -> u32 {
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
                if result == 0 {
                    result = 1;
                } else {
                    result *= 2;
                }
            }
        });
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../inputs/test_input_part_1.txt");
        assert_eq!(process(input), 13);
    }

    #[test]
    fn my_input() {
        let input = include_str!("../inputs/my_input_part_1.txt");
        let result = process(input);
        assert_eq!(result, 23941);
    }
}
