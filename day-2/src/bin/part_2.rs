macro_rules! compare_color {
    ($struct:ident,$color:ident , $value:expr  ) => {
        if $struct.$color < $value {
            $struct.$color = $value;
        }
    };
}

#[derive(Default, Debug)]
struct FewestColors {
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let input = include_str!("../inputs/my_input_part_2.txt");
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
  Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
*/
fn process_line(input: &str) -> u32 {
    let header_index = input.find(":").unwrap();

    // Gets the "3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green" part
    let input = input[header_index + 2..].trim();

    let power = process_block(input);

    return power;
}

/*
 The Block is: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

 return true if the block is valid
*/
fn process_block(mut input: &str) -> u32 {
    let mut result = 0;
    let mut fewest_colors = FewestColors::default();
    while let Some(idx) = input.find(";") {
        let (current, rest) = input.split_at(idx + 1);
        let current = current.trim();
        let rest = rest.trim();
        process_grab(current, &mut fewest_colors);
        input = rest;
    }
    process_grab(input, &mut fewest_colors);
    result += fewest_colors.red * fewest_colors.green * fewest_colors.blue;

    return result;
}

/*
 The Grab is: 3 blue, 4 red
*/
fn process_grab(mut input: &str, fewest_color: &mut FewestColors) {
    input = input.split(";").nth(0).unwrap();
    while let Some(idx) = input.find(",") {
        let (current, rest) = input.split_at(idx + 1);
        let current = current.split(",").nth(0).unwrap();
        let (number, color) = get_number_and_color(current);
        match color {
            "red" => compare_color!(fewest_color, red, number),
            "green" => compare_color!(fewest_color, green, number),
            "blue" => compare_color!(fewest_color, blue, number),
            _ => panic!("Unknown color: {}", color),
        }

        let rest = rest.trim();
        input = rest;
    }
    let (number, color) = get_number_and_color(input);
    match color {
        "red" => compare_color!(fewest_color, red, number),
        "green" => compare_color!(fewest_color, green, number),
        "blue" => compare_color!(fewest_color, blue, number),
        _ => panic!("Unknown color: {}", color),
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_line() {
        let input = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        assert_eq!(process(input), 1560);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../inputs/test_input_part_2.txt");
        assert_eq!(process(input), 2286);
    }

    #[test]
    fn my_input() {
        let input = include_str!("../inputs/my_input_part_2.txt");
        let result = process(input);
        assert_eq!(result, 63307);
    }
}
