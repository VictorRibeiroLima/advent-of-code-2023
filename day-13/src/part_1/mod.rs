use crate::parse::parse_input;

pub fn process(input: &str) -> usize {
    let inputs = parse_input(input);
    let mut horizontal_reflection_count: usize = 0;
    let mut vertical_reflection_count: usize = 0;
    for input in inputs {
        let h = horizontal_reflection(&input);
        if h > 0 {
            horizontal_reflection_count += h;
            continue;
        }
        let v = vertical_reflection(&input);
        if v > 0 {
            vertical_reflection_count += v;
            continue;
        }
    }
    horizontal_reflection_count + (vertical_reflection_count * 100)
}

fn vertical_reflection(input: &Vec<String>) -> usize {
    for i in 1..input.len() {
        let (first_half, second_half) = split_vector(input, i);
        let first_half: Vec<String> = first_half.iter().cloned().collect();
        let second_half: Vec<String> = second_half.iter().rev().cloned().collect();
        if first_half == second_half {
            return i;
        }
    }
    0
}

fn horizontal_reflection(input: &Vec<String>) -> usize {
    let string = &input[0];
    for i in 1..string.len() {
        let reflection = check_horizontal_reflection(&string, i);
        if reflection {
            let result = confirm_horizontal_reflection(input, i);
            if result {
                return i;
            }
        }
    }
    0
}

fn check_horizontal_reflection(string: &String, start: usize) -> bool {
    let string = string.chars().collect::<Vec<char>>();
    let (first_half, second_half) = split_vector(&string, start);
    let first_half = first_half.iter().cloned().collect::<Vec<char>>();
    let second_half = second_half.iter().cloned().rev().collect::<Vec<char>>();

    if first_half == second_half {
        return true;
    }

    false
}

fn confirm_horizontal_reflection(input: &Vec<String>, start: usize) -> bool {
    for i in 1..input.len() {
        let string = &input[i];
        let reflection = check_horizontal_reflection(&string, start);
        if !reflection {
            return false;
        }
    }
    true
}

fn split_vector<T>(input: &Vec<T>, start: usize) -> (&[T], &[T]) {
    let i = start;
    let mut first_half_start = 0;
    let mut first_half_end = i;
    let second_half_start = i;
    let mut second_half_end: usize = input.len();
    let first_half_len = i as isize;
    let second_half_len = (input.len() - i) as isize;
    let len_diff = first_half_len - second_half_len;
    if len_diff < 0 {
        second_half_end = second_half_end - len_diff.abs() as usize;
    } else if len_diff > 0 {
        first_half_start = len_diff as usize;
        first_half_end = second_half_start;
    }
    let first_half = &input[first_half_start..first_half_end];
    let second_half = &input[second_half_start..second_half_end];
    (first_half, second_half)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_horizontal_reflection() {
        let input = "###....###...
...#..#....#.
..........###
.###..###.#.#
...#..#...###
###.##.####..
###.##.###..#
#.#....#.#.#.
...####.....#
##......####.
##......##.#.
#.#.##.#.##..
#.##..##.#.##
#.#....#.#.##
#.#.##.#.##..
##......##.#.
##......####.";
        let input = parse_input(input);
        let input = &input[0];
        let result = horizontal_reflection(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_horizontal_reflection2() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let input = parse_input(input);
        let input = &input[0];
        let result = horizontal_reflection(input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_horizontal_reflection3() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let input = parse_input(input);
        let input = &input[0];
        let result = horizontal_reflection(input);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_vertical_reflection() {
        let input = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let input = parse_input(input);
        let input = &input[0];
        let result = vertical_reflection(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_vertical_reflection2() {
        let input = "#.#...#...##...##
#.#...#...##...##
#....##....#.....
##...##...##.##..
#..##..####....##
.....#.##..#.###.
#.......###..#...
..####.#.#.###..#
#..###.#.#.#####.
.....#.####...###
.....#.####...###";
        let input = parse_input(input);
        let input = &input[0];
        let result = vertical_reflection(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 405);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 30705);
    }
}
