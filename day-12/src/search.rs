use std::collections::HashMap;

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct SearchInput {
    pub numbers: Vec<usize>,
    pub spaces: Vec<u8>,
    pub start: usize,
}

pub fn search(inputs: Vec<(Vec<usize>, Vec<u8>)>) -> usize {
    let mut result = 0;
    let mut memo: HashMap<SearchInput, usize> = HashMap::new();
    for input in inputs {
        let numbers = input.0;
        let spaces = input.1;
        let search_input = SearchInput {
            numbers,
            spaces,
            start: 0,
        };
        result += inner_search(search_input, &mut memo);
    }
    result
}

fn inner_search(search_input: SearchInput, memo: &mut HashMap<SearchInput, usize>) -> usize {
    if let Some(&result) = memo.get(&search_input) {
        return result;
    }
    let numbers = &search_input.numbers;
    let spaces = &search_input.spaces;
    let start = search_input.start;
    let numbers_len = numbers.len();
    let target = numbers[0];
    let mut result = 0;

    let mut continues_number = 0;

    let mut i = start;
    while i < spaces.len() {
        let mut num_of_jumps = 1;
        let mut fonded = false;
        let c = spaces[i] as char;
        match c {
            '?' => {
                continues_number += 1;
            }
            '#' => {
                continues_number += 1;
            }
            '.' => {
                continues_number = 0;
            }

            _ => panic!("unexpected char"),
        }

        if continues_number >= target {
            fonded = check_fonded(target, i, &spaces);
            num_of_jumps += 1;
        }

        if fonded {
            if know_value_until_target_i(&spaces, start, target, i) {
                break;
            }
            if numbers_len == 1 {
                if !know_value_after_target_i(&spaces, i) {
                    result += 1;
                    continues_number = 0;
                    i -= target - 1;
                }
            } else {
                let numbers = numbers[1..].to_vec();
                let search_input = SearchInput {
                    numbers,
                    spaces: spaces.clone(),
                    start: i + num_of_jumps,
                };
                let sub_result = inner_search(search_input, memo);
                if sub_result > 0 {
                    result += sub_result;
                }
                continues_number = 0;
                i -= target - 1;
            }
        }
        i += 1;
    }

    memo.insert(search_input, result);
    result
}

fn know_value_until_target_i(spaces: &Vec<u8>, start: usize, target: usize, i: usize) -> bool {
    if i + 1 == target {
        return false;
    }
    let first_target_index = (i - target) + 1;
    for i in start..first_target_index {
        let c = spaces[i] as char;
        match c {
            '#' => return true,

            _ => {}
        }
    }

    false
}

fn know_value_after_target_i(spaces: &Vec<u8>, i: usize) -> bool {
    let i = i + 1;
    for i in i..spaces.len() {
        let c = spaces[i] as char;
        match c {
            '#' => return true,

            _ => {}
        }
    }
    return false;
}

fn check_fonded(num_len: usize, i: usize, spaces: &Vec<u8>) -> bool {
    let space_len = spaces.len();
    let behind = i as isize - num_len as isize;
    let space_behind = if behind < 0 {
        true
    } else {
        let b = spaces[behind as usize] as char;
        b == '.' || b == '?'
    };

    let ahead = i + 1;
    let space_ahead = if ahead > (space_len - 1) {
        true
    } else {
        let a = spaces[ahead] as char;
        a == '.' || a == '?'
    };

    space_behind && space_ahead
}
