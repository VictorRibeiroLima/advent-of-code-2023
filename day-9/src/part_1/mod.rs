pub fn process(input: &str) -> i32 {
    let mut result = 0;
    let mut lines = Vec::new();
    for line in input.lines() {
        let line = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        lines.push(line);
    }

    for line in lines {
        let num = process_line(line);
        result += num;
    }

    return result;
}

fn process_line(line: Vec<i32>) -> i32 {
    let mut i = 1;
    let mut results = Vec::new();
    results.push(line);
    let mut line = results.get(0).unwrap();
    loop {
        let (new_line, end_line) = gen_line(line);
        if end_line {
            break;
        }
        results.push(new_line);
        line = results.get(i).unwrap();
        i += 1;
    }

    let mut last_num = 0;

    while let Some(mut line) = results.pop() {
        let num = line.pop().unwrap();
        last_num = last_num + num;
    }

    return last_num;
}

fn gen_line(line: &Vec<i32>) -> (Vec<i32>, bool) {
    let mut end_line = true;
    let mut new_line = Vec::new();
    let mut i = 0;
    while i < line.len() - 1 {
        let j = i + 1;
        let num1 = line[j];
        let num2 = line[i];
        let result = num1 - num2;
        end_line = end_line && result == 0;
        new_line.push(result);
        i += 1;
    }
    return (new_line, end_line);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1_line() {
        let input = "0 3 6 9 12 15";
        let result = super::process(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_input.txt");
        let result = super::process(input);
        assert_eq!(result, 114);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../inputs/my_input.txt");
        let result = super::process(input);
        assert_eq!(result, 1934898178);
    }
}
