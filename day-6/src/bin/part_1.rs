#[derive(Debug)]
pub struct Race {
    pub time: u32,
    pub distance: u32,
}

fn main() {
    let input = include_str!("../inputs/my_input.txt");
    println!("Result: {}", process(input));
}

fn process(input: &str) -> u32 {
    let mut result = 0;
    let mut lines = input.lines();
    let mut times = lines.nth(0).unwrap();
    let mut distances = lines.nth(0).unwrap();
    (_, times) = times.split_at(5);
    (_, distances) = distances.split_at(11);
    times = times.trim();
    distances = distances.trim();
    let times = times.split_whitespace().map(|s| s.parse::<u32>().unwrap());
    let distances = distances
        .split_whitespace()
        .map(|s| s.parse::<u32>().unwrap());

    let races = times.zip(distances);
    for (time, distance) in races {
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
        let races_won = time - (holds_until_winning * 2) + 1;
        if result == 0 {
            result = races_won;
        } else if races_won > 0 {
            result *= races_won;
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../inputs/test_input.txt");
        assert_eq!(process(input), 288);
    }

    #[test]
    fn part_1() {
        let input = include_str!("../inputs/my_input.txt");
        assert_eq!(process(input), 170000);
    }
}
