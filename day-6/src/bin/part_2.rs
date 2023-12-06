#[derive(Debug)]
pub struct Race {
    pub time: u64,
    pub distance: u64,
}

fn main() {
    let input = include_str!("../inputs/my_input.txt");
    println!("Result: {}", process(input));
}

fn process(input: &str) -> u64 {
    let mut lines = input.lines();
    let mut times = lines.nth(0).unwrap();
    let mut distances = lines.nth(0).unwrap();
    (_, times) = times.split_at(5);
    (_, distances) = distances.split_at(11);
    let time_chars = times.split_whitespace().map(|s| s.chars());
    let distance_chars = distances.split_whitespace().map(|s| s.chars());

    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for chars in time_chars {
        for c in chars {
            if c.is_digit(10) {
                if time == 0 {
                    time = c.to_digit(10).unwrap() as u64;
                } else {
                    time *= 10;
                    time += c.to_digit(10).unwrap() as u64;
                }
            }
        }
    }

    for chars in distance_chars {
        for c in chars {
            if c.is_digit(10) {
                if distance == 0 {
                    distance = c.to_digit(10).unwrap() as u64;
                } else {
                    distance *= 10;
                    distance += c.to_digit(10).unwrap() as u64;
                }
            }
        }
    }

    let race = Race { time, distance };

    let mut holds_until_winning = 0;

    for hold in 0..=race.time {
        let time_remaining = race.time - hold;
        let travel_distance = hold * time_remaining;

        if travel_distance > race.distance {
            break;
        }
        holds_until_winning += 1;
    }
    let result = time - (holds_until_winning * 2) + 1;

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../inputs/test_input.txt");
        assert_eq!(process(input), 71503);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../inputs/my_input.txt");
        assert_eq!(process(input), 20537782);
    }
}
