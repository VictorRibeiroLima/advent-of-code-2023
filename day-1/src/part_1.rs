pub fn process(input: &str) -> u32 {
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
        let first = first.to_digit(10).unwrap();
        let last = last.to_digit(10).unwrap();
        let digit = first * 10 + last;

        result += digit;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 142);
    }

    #[test]
    fn my_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 52974);
    }
}
