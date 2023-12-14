#[derive(Default, Debug)]
struct Block {
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
    bolder_count: usize,
}

pub fn process(input: &str) -> usize {
    let mut result = 0;
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    tilt_up(&mut lines);
    let lines_len = lines.len();
    for (i, line) in lines.into_iter().enumerate() {
        let height = lines_len - i;
        let mut count = 0;
        for c in line.chars() {
            if c == 'O' {
                count += 1;
            }
        }
        result += count * height;
    }

    result
}

fn tilt_up(lines: &mut Vec<String>) {
    let len = lines[0].len();
    for i in 0..len {
        let mut blocks = Vec::new();
        let mut block = Block::default();
        for (j, line) in lines.iter().enumerate() {
            let c = line.chars().nth(i).unwrap();
            if c == '.' {
                if block.start.is_none() {
                    block.start = Some((j, i));
                }
            } else if c == 'O' {
                if block.start.is_some() {
                    block.bolder_count += 1;
                }
            } else {
                if block.start.is_some() {
                    block.end = Some((j, i));
                    blocks.push(block);
                    block = Block::default();
                }
            }
        }
        if block.start.is_some() {
            block.end = Some((len, i));
            blocks.push(block);
        }
        for block in blocks {
            let start = block.start.unwrap();
            let end = block.end.unwrap();
            let line_start = start.0;
            let line_end = end.0;
            let fixed_index = start.1;
            let bolder_end = line_start + block.bolder_count;
            for i in line_start..bolder_end {
                let line = &mut lines[i];
                swap(line, fixed_index, 'O');
            }
            for i in bolder_end..line_end {
                let line = &mut lines[i];
                swap(line, fixed_index, '.');
            }
        }
    }
}

fn swap(input: &mut String, i: usize, insert: char) {
    let vec = unsafe { input.as_mut_vec() };
    vec[i] = insert as u8;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 136);
    }
}
