use crate::map::Map;

pub fn process(input: &str) -> u32 {
    let map = Map::init(input);
    println!("{}", map);
    let result = map.polygon_points.len() / 2;
    return result as u32;
}

#[cfg(test)]
mod test {

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
        assert_eq!(result, 8);
    }

    #[test]
    fn test_my_input() {
        let input = include_str!("../inputs/my_input.txt");
        let result = super::process(input);
        assert_eq!(result, 6856);
    }
}
