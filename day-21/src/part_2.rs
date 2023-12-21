use crate::map::{Location, Map};

const NUMBER_OF_STEPS: usize = 26501364;

pub fn process(input: &str) -> usize {
    let mut map = Map::new(input);
    let size = map.size();
    let start = map.start_location();

    let grid_width = NUMBER_OF_STEPS / size - 1;
    let odd = square(grid_width / 2 * 2 + 1);

    let even = square((grid_width + 1) / 2 * 2);

    let odd_points = map.walk(size * 2);

    let even_points = map.walk(size * 2 - 1);

    let corner_t = map.walk_from_location(Location::new(size - 1, start.j), size - 2);
    let corner_r = map.walk_from_location(Location::new(start.i, 0), size - 2);
    let corner_b = map.walk_from_location(Location::new(0, start.j), size - 2);
    let corner_l = map.walk_from_location(Location::new(start.i, size - 1), size - 2);

    let small_tr = map.walk_from_location(Location::new(size - 1, 0), size / 2 - 2);
    let small_tl = map.walk_from_location(Location::new(size - 1, size - 1), size / 2 - 2);
    let small_br = map.walk_from_location(Location::new(0, 0), size / 2 - 2);
    let small_bl = map.walk_from_location(Location::new(0, size - 1), size / 2 - 2);

    let large_tr = map.walk_from_location(Location::new(size - 1, 0), size * 3 / 2 - 2);
    let large_tl = map.walk_from_location(Location::new(size - 1, size - 1), size * 3 / 2 - 2);
    let large_br = map.walk_from_location(Location::new(0, 0), size * 3 / 2 - 2);
    let large_bl = map.walk_from_location(Location::new(0, size - 1), size * 3 / 2 - 2);

    let odd = odd * odd_points;
    let even = even * even_points;
    let corner_sum = corner_t + corner_r + corner_b + corner_l;
    let small_sum = small_tr + small_tl + small_br + small_bl;
    let small = (grid_width + 1) * small_sum;
    let large_sum = large_tr + large_tl + large_br + large_bl;
    let large = grid_width * large_sum;

    let result = odd + even + corner_sum + small + large;
    result
}

fn square(size: usize) -> usize {
    size * size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 636_350_496_972_143);
    }
}
