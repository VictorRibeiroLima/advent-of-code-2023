use std::collections::HashMap;

use crate::parse::parse_input;

#[derive(Debug, Default)]
struct HorizontalMemo {
    inner: HashMap<Vec<String>, InnerHorizontalMemo>,
    string_reflection: HashMap<String, HashMap<usize, bool>>,
}

impl HorizontalMemo {
    fn get_result(&self, input: &Vec<String>) -> usize {
        if self.inner.contains_key(input) {
            let mem = self.inner.get(input).unwrap();
            if mem.count.is_some() {
                return mem.count.unwrap();
            }
        }
        0
    }
}

#[derive(Debug, Default)]
struct InnerHorizontalMemo {
    count: Option<usize>,
    result_map: HashMap<usize, bool>,
}

pub fn process(input: &str) -> usize {
    let inputs = parse_input(input);
    let mut horizontal_reflection_count: usize = 0;
    let mut vertical_reflection_count: usize = 0;
    let mut memo_horizontal = HorizontalMemo::default();
    let mut memo_vertical: HashMap<Vec<String>, usize> = HashMap::new();

    for input in inputs {
        process_without_swap(&input, &mut memo_horizontal, &mut memo_vertical);
        let (h, v) = process_input(input, &mut memo_horizontal, &mut memo_vertical);

        if h == 0 && v > 0 {
            vertical_reflection_count += v;
        } else if v == 0 && h > 0 {
            horizontal_reflection_count += h;
        } else if h < v {
            horizontal_reflection_count += h;
        } else {
            vertical_reflection_count += v;
        }
    }
    horizontal_reflection_count + (vertical_reflection_count * 100)
}

fn process_without_swap(
    input: &Vec<String>,
    memo_horizontal: &mut HorizontalMemo,
    memo_vertical: &mut HashMap<Vec<String>, usize>,
) {
    horizontal_reflection(input, memo_horizontal, None);

    vertical_reflection(input, memo_vertical, None);
}

//Horizontal , Vertical
fn process_input(
    mut input: Vec<String>,
    memo_horizontal: &mut HorizontalMemo,
    memo_vertical: &mut HashMap<Vec<String>, usize>,
) -> (usize, usize) {
    let forbidden_h: usize = memo_horizontal.get_result(&input);
    let forbidden_v: usize = *memo_vertical.get(&input).unwrap_or(&0);

    for i in 0..input.len() {
        let string_len = input[i].len();
        for j in 0..string_len {
            let string = &mut input[i];
            swap(string, j);
            let h = horizontal_reflection(&input, memo_horizontal, Some(forbidden_h));
            let v = vertical_reflection(&input, memo_vertical, Some(forbidden_v));

            if h == 0 && v == 0 {
                let string = &mut input[i];
                swap(string, j);
                continue;
            }

            if h > 0 {
                if v == 0 {
                    return (h, 0);
                } else if h < v {
                    return (h, 0);
                }
            }

            if v > 0 {
                if h == 0 {
                    return (0, v);
                } else if v < h {
                    return (0, v);
                }
            }

            let string = &mut input[i];
            swap(string, j);
        }
    }
    (0, 0)
}

fn swap(input: &mut String, i: usize) {
    let vec = unsafe { input.as_mut_vec() };
    let c = vec[i];
    if c == b'#' {
        vec[i] = b'.';
    } else {
        vec[i] = b'#';
    }
}

fn vertical_reflection(
    input: &Vec<String>,
    memo: &mut HashMap<Vec<String>, usize>,
    forbidden_value: Option<usize>,
) -> usize {
    if memo.contains_key(input) {
        let value = *memo.get(input).unwrap();
        match forbidden_value {
            Some(forbidden) => {
                if value != forbidden {
                    return value;
                }
            }
            None => {
                return value;
            }
        }
    }
    let mut result = 0;
    for i in 1..input.len() {
        let (first_half, second_half) = split_vector(input, i);
        let first_half: Vec<String> = first_half.iter().cloned().collect();
        let second_half: Vec<String> = second_half.iter().rev().cloned().collect();
        if first_half == second_half {
            match forbidden_value {
                Some(forbidden) => {
                    if i != forbidden {
                        result = i;
                        break;
                    }
                }
                None => {
                    result = i;
                    break;
                }
            }
        }
    }
    memo.insert(input.clone(), result);
    result
}

fn horizontal_reflection(
    input: &Vec<String>,
    memo: &mut HorizontalMemo,
    forbidden_value: Option<usize>,
) -> usize {
    if memo.inner.contains_key(input) {
        let mem = memo.inner.get(input).unwrap();
        if mem.count.is_some() {
            let value = mem.count.unwrap();
            match forbidden_value {
                Some(forbidden) => {
                    if value != forbidden {
                        return value;
                    }
                }
                None => {
                    return value;
                }
            }
        }
    } else {
        memo.inner
            .insert(input.clone(), InnerHorizontalMemo::default());
    }
    let string = &input[0];
    let mut result = 0;
    for i in 1..string.len() {
        let reflection = check_string_horizontal_reflection(&string, i, memo);
        if reflection {
            let reflected = confirm_horizontal_reflection(input, i, memo);
            if reflected {
                match forbidden_value {
                    Some(forbidden) => {
                        if i != forbidden {
                            result = i;
                            break;
                        }
                    }
                    None => {
                        result = i;
                        break;
                    }
                }
            }
        }
    }
    memo.inner.get_mut(input).unwrap().count = Some(result);
    result
}

fn check_string_horizontal_reflection(
    input: &String,
    start: usize,
    memo: &mut HorizontalMemo,
) -> bool {
    if memo.string_reflection.contains_key(input) {
        let mem = memo.string_reflection.get(input).unwrap();
        if mem.contains_key(&start) {
            return *mem.get(&start).unwrap();
        }
    } else {
        memo.string_reflection.insert(input.clone(), HashMap::new());
    }
    let string = input.chars().collect::<Vec<char>>();
    let (first_half, second_half) = split_vector(&string, start);
    let first_half = first_half.iter().cloned().collect::<Vec<char>>();
    let second_half = second_half.iter().cloned().rev().collect::<Vec<char>>();

    if first_half == second_half {
        memo.string_reflection
            .get_mut(input)
            .unwrap()
            .insert(start, true);
        return true;
    }

    memo.string_reflection
        .get_mut(input)
        .unwrap()
        .insert(start, false);
    false
}

fn confirm_horizontal_reflection(
    input: &Vec<String>,
    start: usize,
    memo: &mut HorizontalMemo,
) -> bool {
    if memo.inner.contains_key(input) {
        let mem = memo.inner.get(input).unwrap();
        if mem.result_map.contains_key(&start) {
            return *mem.result_map.get(&start).unwrap();
        }
    }

    for i in 1..input.len() {
        let string = &input[i];
        let reflection = check_string_horizontal_reflection(&string, start, memo);
        if !reflection {
            memo.inner
                .get_mut(input)
                .unwrap()
                .result_map
                .insert(start, false);
            return false;
        }
    }
    memo.inner
        .get_mut(input)
        .unwrap()
        .result_map
        .insert(start, true);
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
    fn test_input() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 400);
    }

    #[test]
    fn edge_case() {
        let input = include_str!("../inputs/edge.txt");

        let result = process(input);
        assert_eq!(result, 1200);
    }

    #[test]
    fn edge_case2() {
        let input = include_str!("../inputs/edge_2.txt");

        let result = process(input);
        assert_eq!(result, 1000);
    }

    #[test]
    fn edge_case3() {
        let input = include_str!("../inputs/edge_3.txt");

        let result = process(input);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 44615);
    }
}
