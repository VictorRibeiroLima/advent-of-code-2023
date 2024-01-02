use std::collections::HashSet;

#[derive(Debug)]
struct Card {
    points: u32,
}

impl Card {
    fn new(line: &str) -> Self {
        let mut wins = 0;
        let (_, numbers) = line.split_at(line.find(":").unwrap() + 2);
        let (my_numbers, winning_numbers) = numbers.split_at(numbers.find("|").unwrap());
        let my_numbers = my_numbers.trim();
        let winning_numbers = winning_numbers[1..].trim();

        let my_numbers: HashSet<u32> = my_numbers
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect();

        winning_numbers
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .for_each(|x| {
                if my_numbers.contains(&x) {
                    wins += 1;
                }
            });

        let points: u32 = if wins == 0 {
            0
        } else {
            2_u32.pow(wins as u32 - 1)
        };

        return Card { points };
    }
}

pub fn process(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let card = Card::new(line);
        result += card.points;
    }

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./inputs/test.txt");
        assert_eq!(process(input), 13);
    }

    #[test]
    fn my_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 23941);
    }
}
