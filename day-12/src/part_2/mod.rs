use crate::search::search;

pub fn process(input: &str) -> usize {
    let mut inputs_vec = Vec::new();
    for line in input.lines() {
        let mut inputs = line.split_whitespace();
        let spaces = inputs.next().unwrap().as_bytes().to_vec();
        let number = inputs.next().unwrap();
        let numbers = number
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        let mut final_spaces = Vec::new();
        let mut final_numbers = Vec::new();
        for i in 0..5 {
            for space in &spaces {
                final_spaces.push(*space);
            }
            for number in &numbers {
                final_numbers.push(*number);
            }
            if i != 4 {
                final_spaces.push(b'?');
            }
        }

        inputs_vec.push((final_numbers, final_spaces));
    }

    search(inputs_vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edge_case() {
        let input = "??#???#?##??. 4,5";
        assert_eq!(process(input), 1875);
    }

    #[test]
    fn edge_case2() {
        let input = "??#???.???? 2,4";
        assert_eq!(process(input), 10272);
    }

    #[test]
    fn edge_case3() {
        let input = "??#??? 2";
        assert_eq!(process(input), 32);
    }

    #[test]
    fn edge_case4() {
        let input = "???#????#??? 2,4";
        assert_eq!(process(input), 16807);
    }

    #[test]
    fn test_process() {
        let input = "???.### 1,1,3";
        assert_eq!(process(input), 1);
    }

    #[test]
    fn test_process2() {
        let input = ".??..??...?##. 1,1,3";
        assert_eq!(process(input), 16384);
    }

    #[test]
    fn test_process3() {
        let input = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(process(input), 1);
    }

    #[test]
    fn test_process4() {
        let input = "????.#...#... 4,1,1";
        assert_eq!(process(input), 16);
    }

    #[test]
    fn test_process5() {
        let input = "????.######..#####. 1,6,5";
        assert_eq!(process(input), 4);
    }

    #[test]
    fn test_process6() {
        let input = "?###???????? 3,2,1";
        assert_eq!(process(input), 2500);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test.txt");
        assert_eq!(process(input), 525152);
    }

    #[test]
    fn test_real_input() {
        let input = include_str!("../inputs/input.txt");
        assert_eq!(process(input), 1566786613613);
    }
}
