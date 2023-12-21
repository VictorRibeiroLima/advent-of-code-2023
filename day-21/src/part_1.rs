use crate::map::Map;

pub fn process(input: &str) -> usize {
    let mut map = Map::new(input);
    let result = map.walk(63);
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 3858);
    }
}
