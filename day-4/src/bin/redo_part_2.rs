use std::collections::HashSet;

#[derive(Debug)]
#[allow(dead_code)]
struct Card {
    wins: usize,
    copies: u32,
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

        let points: u32 = if wins == 0 { 0 } else { 2_u32.pow(wins as u32) };

        return Card {
            wins,
            copies: 1,
            points,
        };
    }
}

fn main() {
    let input = include_str!("../inputs/my_input_part_2.txt");
    let result = process(input);
    println!("Result: {}", result);
}

fn process(input: &str) -> u32 {
    let mut cards: Vec<Card> = Vec::new();
    let mut result = 0;
    for line in input.lines() {
        let card = Card::new(line);
        cards.push(card);
    }

    for i in 0..cards.len() {
        let card = cards.get(i).unwrap();
        let copies = card.copies;
        let wins = card.wins;
        result += copies;

        if wins == 0 {
            continue;
        }

        for j in (i + 1)..i + wins + 1 {
            if j >= cards.len() {
                break;
            }
            cards.get_mut(j).unwrap().copies += copies;
        }
    }
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
