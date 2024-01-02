const MAX_NUMBER_OF_RED: u32 = 12;
const MAX_NUMBER_OF_GREEN: u32 = 13;
const MAX_NUMBER_OF_BLUE: u32 = 14;

pub fn process(input: &str) -> u32 {
    let mut result = 0;
    for line in input.lines() {
        let game_id = process_line(line);

        result += game_id;
    }
    return result;
}

/*Original:
  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
*/
fn process_line(input: &str) -> u32 {
    let header_index = input.find(":").unwrap();
    // Gets the "Game x" part
    let header = &input[0..header_index];
    // Gets the x part and converts it to a u32
    let game = header.split(" ").nth(1).unwrap().parse::<u32>().unwrap();

    // Gets the "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" part
    let input = input[header_index + 2..].trim();

    let valid_game = process_block(input);

    if valid_game {
        return game;
    }
    return 0;
}

/*
 The Block is: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

 return true if the block is valid
*/
fn process_block(mut input: &str) -> bool {
    while let Some(idx) = input.find(";") {
        let (current, rest) = input.split_at(idx + 1);
        let current = current.trim();
        let rest = rest.trim();
        let result = process_grab(current);
        if !result {
            return false;
        }

        input = rest;
    }
    let result = process_grab(input);
    if !result {
        return false;
    }
    return true;
}

/*
 The Grab is: 3 blue, 4 red

 return true if the grab is valid
*/
fn process_grab(mut input: &str) -> bool {
    input = input.split(";").nth(0).unwrap();
    while let Some(idx) = input.find(",") {
        let (current, rest) = input.split_at(idx + 1);
        let current = current.split(",").nth(0).unwrap();
        let (number, color) = get_number_and_color(current);
        let max = color_max(color);
        if number > max {
            return false;
        }

        let rest = rest.trim();
        input = rest;
    }
    let (number, color) = get_number_and_color(input);
    let max = color_max(color);
    if number > max {
        return false;
    }

    return true;
}

#[inline]
fn get_number_and_color(input: &str) -> (u32, &str) {
    let input = input.trim();
    let (number, color) = input.split_at(input.find(" ").unwrap());
    let number = number.trim().parse::<u32>().unwrap();
    let (color, _) = color.split_at(color.len());
    let color = color.trim();
    return (number, color);
}

fn color_max(color: &str) -> u32 {
    match color {
        "red" => MAX_NUMBER_OF_RED,
        "green" => MAX_NUMBER_OF_GREEN,
        "blue" => MAX_NUMBER_OF_BLUE,
        _ => panic!("Invalid color"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_line() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(process(input), 0);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("./inputs/test.txt");
        assert_eq!(process(input), 8);
    }

    #[test]
    fn my_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 2416);
    }
}
