use std::sync::Arc;

use map::{MapRange, Maps};

mod map;

const NUMBER_OF_THREADS: u32 = 16;
fn main() {
    let input = include_str!("../inputs/my_input_part_2.txt");
    let result = process(input);
    println!("result: {}", result);
}

fn process(input: &str) -> u32 {
    let (seeds_input, input) = input.split_at(input.find("\n").unwrap());
    let seeds_input = seeds_input[seeds_input.find(":").unwrap() + 1..].trim();
    let mut seed_ranges: Vec<MapRange> = vec![];
    let maps = Maps::new(input);

    let mut lower_bound = maps.nest_lower_bound(0);

    let mut start = 0;

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

    let seed_ranges = Arc::new(seed_ranges);
    let maps = Arc::new(maps);

    let mut lowest_location = search_lowest(seed_ranges.clone(), maps.clone(), 0, lower_bound);

    while lowest_location.is_none() {
        lower_bound = maps.as_ref().nest_lower_bound(lower_bound);
        lowest_location = search_lowest(seed_ranges.clone(), maps.clone(), 0, lower_bound);
    }

    return lowest_location.unwrap();
}

/*  This is in general a much better solution than the one in day-5-redo/src/bin/part_2.rs.
    The idea is to loop over every possible lowest location, and check if it is in the seed ranges.
    Slipt the work into NUMBER_OF_THREADS parts, and use a thread for each part.

    This can be much slower than the original solution in the worst case:
        when the seed range is soo far off that it takes multiple iterations to find a match.

    But is much faster in every other case.
*/
fn search_lowest(
    seed_ranges: Arc<Vec<MapRange>>,
    maps: Arc<Maps>,
    mut start: u32,
    mut end: u32,
) -> Option<u32> {
    let range = end - start;
    if range % NUMBER_OF_THREADS != 0 {
        end += NUMBER_OF_THREADS - range % NUMBER_OF_THREADS;
    }
    let mid = (start + end) / NUMBER_OF_THREADS;
    let mut join_handles = vec![];

    while start < end {
        let mut target = start + mid;
        if target == end {
            target += 1;
        }
        let maps = maps.clone();
        let seed_ranges = seed_ranges.clone();

        let handle = std::thread::spawn(move || {
            let mut lowest_location = None;
            for location in start..target {
                let seed = maps.location_to_seed(location);
                for seed_range in seed_ranges.as_ref().iter() {
                    if seed >= seed_range.start && seed < seed_range.end {
                        if lowest_location.is_none() {
                            lowest_location = Some(location);
                        } else if location < lowest_location.unwrap() {
                            lowest_location = Some(location);
                        }
                    }
                }
            }
            return lowest_location;
        });
        join_handles.push(handle);

        start += mid;
    }

    let mut lowest_location = None;

    for handle in join_handles {
        let result = handle.join().unwrap();
        match result {
            Some(location) => {
                if lowest_location.is_none() {
                    lowest_location = Some(location);
                } else if location < lowest_location.unwrap() {
                    lowest_location = Some(location);
                }
            }
            None => {}
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
