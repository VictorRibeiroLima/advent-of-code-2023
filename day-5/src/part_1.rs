use crate::map::Maps;

pub fn process(input: &str) -> u32 {
    let mut seeds = Vec::new();

    let (seeds_input, input) = input.split_at(input.find("\n").unwrap());
    let seeds_input = seeds_input[seeds_input.find(":").unwrap() + 1..].trim();

    for seed in seeds_input.split_whitespace() {
        seeds.push(seed.parse::<u32>().unwrap());
    }
    let maps = Maps::new(input);
    let mut lowest_location = 0;
    for (i, seed) in seeds.into_iter().enumerate() {
        let location = maps.seed_to_location(seed);
        if i == 0 {
            lowest_location = location;
        } else if location < lowest_location {
            lowest_location = location;
        }
    }

    return lowest_location;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./inputs/test.txt");
        assert_eq!(process(input), 35);
    }

    #[test]
    fn test_seed_to_location() {
        let input = include_str!("./inputs/test.txt");
        let maps = Maps::new(input);
        let soil = maps.seed_to_location(79);
        assert_eq!(soil, 82);
    }

    #[test]
    fn my_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 525792406);
    }
}
