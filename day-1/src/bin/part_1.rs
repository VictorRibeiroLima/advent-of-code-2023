fn main() {
    let input = include_str!("../inputs/my_input_part_1.txt");
    let result = process(input);
    println!("Result: {}", result)
}

fn process(input: &str) -> i32 {
    let mut result = 0;
    for line in input.lines() {
        let mut first = '\n';
        let mut last = '\n';
        for ch in line.chars() {
            if ch.is_numeric() {
                if first == '\n' {
                    first = ch;
                } else {
                    last = ch;
                }
            }
        }
        if last == '\n' {
            last = first;
        }
        let digit = format!("{}{}", first, last).parse::<i32>().unwrap();

        println!("{} => {}", line, digit);
        result += digit;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_input_part_1.txt");
        let result = process(input);
        assert_eq!(result, 142);
    }
}
