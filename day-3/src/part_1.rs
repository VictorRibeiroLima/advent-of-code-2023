struct LineBlock<'a> {
    last_line: Option<&'a [u8]>,
    line: &'a [u8],
    next_line: Option<&'a [u8]>,
}

pub fn process(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut result = 0;

    for i in 0..lines.len() {
        let line_block = to_line_block(&lines, i);
        for j in 0..line_block.line.len() {
            let line_max_len = line_block.line.len() - 1;
            let current_char = line_block.line[j] as char;
            if current_char == '.' || current_char.is_numeric() {
                continue;
            }
            result += front_and_back(line_block.line, j, line_max_len);

            //Check last line
            if let Some(last_line) = line_block.last_line {
                let line_max_len = last_line.len() - 1;
                let index_number = create_number_from_index(j, last_line);
                if index_number != 0 {
                    result += index_number;
                } else {
                    result += front_and_back(last_line, j, line_max_len);
                }
            }

            //Check next line
            if let Some(next_line) = line_block.next_line {
                let line_max_len = next_line.len() - 1;
                let index_number = create_number_from_index(j, next_line);
                if index_number != 0 {
                    result += index_number;
                } else {
                    result += front_and_back(next_line, j, line_max_len);
                }
            }
        }
    }
    return result;
}

#[inline]
fn front_and_back(line: &[u8], index: usize, max_len: usize) -> u32 {
    let mut result = 0;
    //Check backwards from the symbol
    if index != 0 {
        result += create_number_from_index(index - 1, line);
    }
    //Check forwards from the symbol
    if index != max_len {
        result += create_number_from_index(index + 1, line);
    }
    return result;
}

fn to_line_block<'a>(lines: &Vec<&'a str>, i: usize) -> LineBlock<'a> {
    let last_line = if i == 0 { None } else { Some(lines[i - 1]) };
    let line = lines[i];
    let next_line = if i == lines.len() - 1 {
        None
    } else {
        Some(lines[i + 1])
    };

    return LineBlock {
        last_line: last_line.map(|x| x.as_bytes()),
        line: line.as_bytes(),
        next_line: next_line.map(|x| x.as_bytes()),
    };
}

fn create_number_from_index(mut index: usize, line: &[u8]) -> u32 {
    let mut result = 0;
    let multiplier = 10;
    let input_length = line.len();
    let mut last_digit_index = None;
    loop {
        let current_char = line[index] as char;
        if !current_char.is_numeric() {
            break;
        }
        last_digit_index = Some(index);
        if index == 0 {
            break;
        }
        index -= 1;
    }

    index = if let Some(index) = last_digit_index {
        index
    } else {
        return 0;
    };

    loop {
        if index == input_length {
            break;
        }
        let current_char = line[index] as char;
        if !current_char.is_numeric() {
            break;
        }
        let current_number = current_char.to_digit(10).unwrap();
        result = (result * multiplier) + current_number;
        index += 1;
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_number_from_index() {
        let input = "467..114..".as_bytes();
        let result = create_number_from_index(0, input);
        assert_eq!(result, 467);
        let result = create_number_from_index(1, input);
        assert_eq!(result, 467);
        let result = create_number_from_index(2, input);
        assert_eq!(result, 467);
        let result = create_number_from_index(3, input);
        assert_eq!(result, 0);
        let result = create_number_from_index(4, input);
        assert_eq!(result, 0);
        let result = create_number_from_index(5, input);
        assert_eq!(result, 114);
        let result = create_number_from_index(6, input);
        assert_eq!(result, 114);
        let result = create_number_from_index(7, input);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_process_1_line() {
        let input = "617*.3....";
        let result = process(input);
        assert_eq!(result, 617);

        let input = "6.7*.3....";
        let result = process(input);
        assert_eq!(result, 7);

        let input = "6.7*3..1..";
        let result = process(input);
        assert_eq!(result, 10);

        let input = "617*34.1..";
        let result = process(input);
        assert_eq!(result, 651);
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("./inputs/test.txt");
        assert_eq!(process(input), 4361);
    }

    #[test]
    fn my_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 531561);
    }
}
