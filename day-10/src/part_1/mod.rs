use crate::map::Map;

pub fn process(input: &str) -> u32 {
    let mut map = Map::new(input);

    map.connect_pipes();
    map.prone_unused_pipes();

    println!("{}", map);
    return 0;
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1_line() {
        let input = "F-7";
        let result = super::process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_simple_input() {
        let input = include_str!("../inputs/simple_test_input.txt");
        let result = super::process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_input() {
        let input = include_str!("../inputs/test_input.txt");
        let result = super::process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../inputs/my_input.txt");
        let result = super::process(input);
        assert_eq!(result, 4);
    }
}
