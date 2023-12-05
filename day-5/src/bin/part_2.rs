use std::sync::Arc;

use map::{MapRange, Maps};

mod map;
fn main() {
    let input = include_str!("../inputs/my_input_part_2.txt");
    let result = process(input);
    println!("result: {}", result);
}

fn process(input: &str) -> u32 {
    let (seeds_input, input) = input.split_at(input.find("\n").unwrap());
    let seeds_input = seeds_input[seeds_input.find(":").unwrap() + 1..].trim();
    let mut seed_ranges: Vec<MapRange> = vec![];
    let mut join_handles = vec![];

    let mut start = 0;
    let maps = Maps::new(input);

    let maps = Arc::new(maps);

    for (i, number_str) in seeds_input.split_whitespace().enumerate() {
        let number = number_str.parse::<u32>().unwrap();
        if i % 2 == 0 {
            start = number;
            continue;
        }
        let end = start + number;
        seed_ranges.push(MapRange { start, end });

        start = 0;
    }
    for seed_range in seed_ranges {
        println!("seed_range: {:?}", seed_range);
        let map = maps.clone();
        let join_handle = std::thread::spawn(move || {
            let mut lowest_location = std::u32::MAX;
            for seed in seed_range.start..seed_range.end {
                let location = map.seed_to_location(seed);

                if location < lowest_location {
                    lowest_location = location;
                }
            }
            return lowest_location;
        });
        join_handles.push(join_handle);
    }
    let mut lowest_location = std::u32::MAX;
    println!("Threads created, waiting for them to finish");
    for handle in join_handles {
        let result = handle.join().unwrap();
        if result < lowest_location {
            lowest_location = result;
        }
    }

    return lowest_location;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("../inputs/test_input_part_2.txt");
        assert_eq!(process(input), 46);
    }

    #[test]
    fn test_seed_to_location() {
        let input = include_str!("../inputs/test_input_part_2.txt");
        let maps = Maps::new(input);
        let location = maps.seed_to_location(79);
        assert_eq!(location, 82);
    }

    #[test]
    fn test_seed_to_location2() {
        let input = include_str!("../inputs/my_input_part_2.txt");
        let maps = Maps::new(input);
        let location = maps.seed_to_location(459278395);
        assert_eq!(location, 260579843);
    }

    #[test]
    fn my_input() {
        let input = include_str!("../inputs/my_input_part_2.txt");
        let result = process(input);
        assert_eq!(result, 79004094);
    }
}
