use std::vec;
const NUM_OF_EXP: usize = 999999;

pub fn process(input: &str) -> usize {
    let mut result = 0;
    let lines = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let (empty_i_indexes, empty_j_indexes) = count_empty(&lines);
    let indexes = galaxies_indexes(&lines, &empty_i_indexes, &empty_j_indexes);
    for i in 0..indexes.len() {
        let (i1, j1) = indexes[i];
        for j in i + 1..indexes.len() {
            let (i2, j2) = indexes[j];
            let distance = manhattan_distance(i1 as i32, j1 as i32, i2 as i32, j2 as i32);
            result += distance as usize;
        }
    }
    result
}

fn count_empty(lines: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let j_len = lines[0].len();
    let i_len = lines.len();
    let mut empty_is = vec![false; i_len];
    let mut j_count = vec![0; j_len];
    for i in 0..lines.len() {
        let mut count = 0;
        for j in 0..j_len {
            let c = lines[i][j];
            if c == '#' {
                count += 1;
                j_count[j] += 1;
            }
        }
        if count == 0 {
            empty_is[i] = true;
        }
    }

    let mut char_count: usize = 0;
    let empty_j_indexes = j_count
        .iter()
        .enumerate()
        .filter(|(_, &count)| count == 0)
        .map(|(i, _)| {
            let result = i + char_count;
            char_count += 1;
            result
        })
        .collect::<Vec<usize>>();

    let mut line_count = 0;
    let empty_i_indexes = empty_is
        .iter()
        .enumerate()
        .filter(|(_, &is_empty)| is_empty)
        .map(|(i, _)| {
            let result = i + line_count;
            line_count += 1;
            result
        })
        .collect::<Vec<usize>>();

    return (empty_i_indexes, empty_j_indexes);
}

fn galaxies_indexes(
    lines: &Vec<Vec<char>>,
    empty_i_indexes: &Vec<usize>,
    empty_j_indexes: &Vec<usize>,
) -> Vec<(usize, usize)> {
    let mut indexes = vec![];
    for i in 0..lines.len() {
        for j in 0..lines[0].len() {
            if lines[i][j] == '#' {
                let mut greater_i = 0;
                let mut greater_j = 0;
                for empty_i in empty_i_indexes {
                    if (i + greater_i) >= *empty_i {
                        greater_i += 1;
                    } else {
                        break;
                    }
                }
                for empty_j in empty_j_indexes {
                    if (j + greater_j) >= *empty_j {
                        greater_j += 1;
                    } else {
                        break;
                    }
                }

                let true_i = i + (greater_i * NUM_OF_EXP);
                let true_j = j + (greater_j * NUM_OF_EXP);
                indexes.push((true_i, true_j));
            }
        }
    }
    indexes
}

fn manhattan_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> i32 {
    let dx = (x1 - x2).abs();
    let dy = (y1 - y2).abs();
    dx + dy
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process() {
        let input = include_str!("../inputs/test.txt");
        assert_eq!(process(input), 8410);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(process(input), 699909023130);
    }
}
