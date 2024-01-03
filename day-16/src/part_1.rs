use crate::grid;

pub fn process(input: &str) -> usize {
    let mut grid = grid::Grid::new(input);
    grid.energize();
    grid.count_energized()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("./inputs/test.txt");
        assert_eq!(process(input), 46);
    }

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/input.txt");
        assert_eq!(process(input), 6514);
    }
}
