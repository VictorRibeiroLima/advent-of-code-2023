pub fn process(input: &str) -> usize {
    let mut result = 0;
    for line in input.lines() {
        for part in line.split(',') {
            let mut value = 0;
            for c in part.chars() {
                value += (c as u8) as usize;
                value *= 17;
                value %= 256;
            }
            result += value;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input), 1320);
    }
}
