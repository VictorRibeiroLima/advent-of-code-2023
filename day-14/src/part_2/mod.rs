use std::collections::HashMap;

const TOTAL_CYCLES: u32 = 1_000_000_000;

#[derive(Default, Debug)]
struct Block {
    start: Option<(usize, usize)>,
    end: Option<(usize, usize)>,
    bolder_count: usize,
}

#[derive(Default, Debug)]
struct Memo {
    left_memo: HashMap<String, Vec<String>>,
    right_memo: HashMap<String, Vec<String>>,
    up_memo: HashMap<String, Vec<String>>,
    down_memo: HashMap<String, Vec<String>>,
}

pub fn process(input: &str) -> usize {
    let mut result = 0;
    let mut lines: Vec<String> = input.lines().map(|s| s.to_string()).collect();
    let mut memo: Memo = Memo::default();
    let mut seen = HashMap::new();
    let string = vec_to_string(&lines);
    seen.insert(string, 0);
    let mut cycle_start = 0;
    let mut cycle_len = 0;

    for c in 0..TOTAL_CYCLES {
        cycle(&mut lines, &mut memo);
        let string = vec_to_string(&lines);
        if let Some(prev) = seen.insert(string, c + 1) {
            /*We are in a closed loop
            So 1->2->3->4->3->2->1 ....
            We need to register the start of this loop and the length of it
            */
            cycle_start = prev;
            cycle_len = c - prev + 1;
            break;
        }
    }

    /*
    in the example above:
      1->2->3->4->3->2->1 ....

    we stopped the loop at "1"

    and we need to get to the "4"

     */
    let total_loops = (TOTAL_CYCLES - cycle_start) % cycle_len;

    for _ in 0..total_loops {
        cycle(&mut lines, &mut memo);
    }

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

fn cycle(lines: &mut Vec<String>, memo: &mut Memo) {
    tilt_up(lines, memo);
    tilt_left(lines, memo);
    tilt_down(lines, memo);
    tilt_right(lines, memo);
}

fn tilt_up(lines: &mut Vec<String>, memo: &mut Memo) {
    let string = vec_to_string(&lines);
    if let Some(l) = memo.up_memo.get(&string) {
        *lines = l.clone();
        return;
    }
    let len = lines[0].len();
    for column_i in 0..len {
        let mut blocks = Vec::new();
        let mut block = Block::default();
        for (line_i, line) in lines.iter().enumerate() {
            let c = line.chars().nth(column_i).unwrap();
            if c == '.' {
                if block.start.is_none() {
                    block.start = Some((line_i, column_i));
                }
            } else if c == 'O' {
                if block.start.is_some() {
                    block.bolder_count += 1;
                }
            } else {
                if block.start.is_some() {
                    block.end = Some((line_i, column_i));
                    blocks.push(block);
                    block = Block::default();
                }
            }
        }
        if block.start.is_some() {
            block.end = Some((len, column_i));
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
    memo.up_memo.insert(string, lines.clone());
}

fn tilt_down(lines: &mut Vec<String>, memo: &mut Memo) {
    let string = vec_to_string(&lines);
    if let Some(l) = memo.down_memo.get(&string) {
        *lines = l.clone();
        return;
    }
    let len = lines[0].len();
    let lines_len = lines.len();
    for column_i in 0..len {
        let mut blocks = Vec::new();
        let mut block = Block::default();

        for line_i in (0..lines_len).rev() {
            let line = &lines[line_i];
            let c = line.chars().nth(column_i).unwrap();
            if c == '.' {
                if block.start.is_none() {
                    block.start = Some((line_i, column_i));
                }
            } else if c == 'O' {
                if block.start.is_some() {
                    block.bolder_count += 1;
                }
            } else {
                if block.start.is_some() {
                    block.end = Some((line_i, column_i));
                    blocks.push(block);
                    block = Block::default();
                }
            }
        }
        if block.start.is_some() {
            block.end = Some((0, column_i));
            blocks.push(block);
        }
        for block in blocks {
            let start = block.start.unwrap();
            let end = block.end.unwrap();
            let line_start = start.0;
            let line_end = end.0;
            let fixed_index = start.1;
            let bolder_end = line_start - block.bolder_count;
            for i in bolder_end..=line_start {
                let line = &mut lines[i];
                swap(line, fixed_index, 'O');
            }
            for i in line_end..=bolder_end {
                let line = &mut lines[i];
                swap(line, fixed_index, '.');
            }
        }
    }
    memo.down_memo.insert(string, lines.clone());
}

fn tilt_left(lines: &mut Vec<String>, memo: &mut Memo) {
    let string = vec_to_string(&lines);
    if let Some(l) = memo.left_memo.get(&string) {
        *lines = l.clone();
        return;
    }
    let mut blocks = Vec::new();
    for (line_i, line) in lines.iter().enumerate() {
        let mut block = Block::default();
        for (column_i, c) in line.chars().enumerate() {
            if c == '.' {
                if block.start.is_none() {
                    block.start = Some((line_i, column_i));
                }
            } else if c == 'O' {
                if block.start.is_some() {
                    block.bolder_count += 1;
                }
            } else {
                if block.start.is_some() {
                    block.end = Some((line_i, column_i));
                    blocks.push(block);
                    block = Block::default();
                }
            }
        }
        if block.start.is_some() {
            block.end = Some((line_i, line.len()));
            blocks.push(block);
        }
    }
    for block in blocks {
        let start = block.start.unwrap();
        let end = block.end.unwrap();
        let line_start = start.1;
        let line_end = end.1;
        let fixed_index = start.0;
        let bolder_end = line_start + block.bolder_count;
        for i in line_start..bolder_end {
            let line = &mut lines[fixed_index];
            swap(line, i, 'O');
        }
        for i in bolder_end..line_end {
            let line = &mut lines[fixed_index];
            swap(line, i, '.');
        }
    }
    memo.left_memo.insert(string, lines.clone());
}

fn tilt_right(lines: &mut Vec<String>, memo: &mut Memo) {
    let string = vec_to_string(&lines);
    if let Some(l) = memo.right_memo.get(&string) {
        *lines = l.clone();
        return;
    }
    let mut blocks = Vec::new();
    for (line_i, line) in lines.iter().enumerate() {
        let mut block = Block::default();
        let line_len = line.len();
        for column_i in (0..line_len).rev() {
            let c = line.chars().nth(column_i).unwrap();
            if c == '.' {
                if block.start.is_none() {
                    block.start = Some((line_i, column_i));
                }
            } else if c == 'O' {
                if block.start.is_some() {
                    block.bolder_count += 1;
                }
            } else {
                if block.start.is_some() {
                    block.end = Some((line_i, column_i));
                    blocks.push(block);
                    block = Block::default();
                }
            }
        }
        if block.start.is_some() {
            block.end = Some((line_i, 0));
            blocks.push(block);
        }
    }
    for block in blocks {
        let start = block.start.unwrap();
        let end = block.end.unwrap();
        let line_start = start.1;
        let line_end = end.1;
        let fixed_index = start.0;
        let bolder_end = line_start - block.bolder_count;
        for i in bolder_end..=line_start {
            let line = &mut lines[fixed_index];
            swap(line, i, 'O');
        }
        for i in line_end..=bolder_end {
            let line = &mut lines[fixed_index];
            swap(line, i, '.');
        }
    }
    memo.right_memo.insert(string, lines.clone());
}

fn swap(input: &mut String, i: usize, insert: char) {
    let vec = unsafe { input.as_mut_vec() };
    let c = vec[i];
    if c == b'#' {
        return;
    }
    vec[i] = insert as u8;
}

fn vec_to_string(input: &Vec<String>) -> String {
    input.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part_2() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 64);
    }
}
