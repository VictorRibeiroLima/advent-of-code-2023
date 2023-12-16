use crate::grid::{tile::Movement, Grid};

pub fn process(input: &str) -> usize {
    let grid = Grid::new(input);
    let mut handles = Vec::new();

    let top_grid = grid.clone();
    handles.push(std::thread::spawn(move || count_top_corner(top_grid)));

    let bottom_grid = grid.clone();
    handles.push(std::thread::spawn(move || count_bottom_corner(bottom_grid)));
    let left_grid = grid.clone();
    handles.push(std::thread::spawn(move || count_left_corner(left_grid)));
    handles.push(std::thread::spawn(move || count_right_corner(grid.clone())));

    let mut count = 0;
    for handle in handles {
        let count_energized = handle.join().unwrap();
        if count_energized > count {
            count = count_energized;
        }
    }
    count
}

fn count_top_corner(mut grid: Grid) -> usize {
    let mut count = 0;
    let len: isize = grid.tiles[0].len() as isize;
    for i in 0..len {
        let location = (0, i);
        grid.energize_from(location, Movement::Down);
        let count_energized = grid.count_energized();
        if count_energized > count {
            count = count_energized;
        }
        grid.reset();
    }
    count
}

fn count_bottom_corner(mut grid: Grid) -> usize {
    let mut count = 0;
    let len: isize = grid.tiles[0].len() as isize;
    for i in 0..len {
        grid.energize_from((grid.tiles.len() as isize - 1, i), Movement::Up);
        let count_energized = grid.count_energized();
        if count_energized > count {
            count = count_energized;
        }
        grid.reset();
    }
    count
}

fn count_left_corner(mut grid: Grid) -> usize {
    let mut count = 0;
    let len: isize = grid.tiles.len() as isize;
    for i in 0..len {
        grid.energize_from((i, 0), Movement::Right);
        let count_energized = grid.count_energized();
        if count_energized > count {
            count = count_energized;
        }
        grid.reset();
    }
    count
}

fn count_right_corner(mut grid: Grid) -> usize {
    let mut count = 0;
    let len: isize = grid.tiles.len() as isize;
    for i in 0..len {
        grid.energize_from((i, grid.tiles[0].len() as isize - 1), Movement::Left);
        let count_energized = grid.count_energized();
        if count_energized > count {
            count = count_energized;
        }
        grid.reset();
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("./inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 51);
    }

    #[test]
    fn test_input() {
        let input = include_str!("./inputs/input.txt");
        assert_eq!(process(input), 8089);
    }
}
