use hand::Hand;

mod hand;

pub fn process(input: &str) -> u32 {
    let mut result = 0;
    let mut hands: Vec<Hand> = Vec::new();
    for line in input.lines() {
        let hand = Hand::new(line);
        hands.push(hand);
    }
    hands.sort();
    for (i, hand) in hands.into_iter().enumerate() {
        result += hand.bid * (i as u32 + 1)
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../inputs/test_input.txt");
        let result = process(input);
        assert_eq!(result, 6440);
    }

    #[test]
    fn test_process_2() {
        let input = include_str!("../inputs/my_input.txt");
        let result = process(input);
        assert_eq!(result, 249390788);
    }
}