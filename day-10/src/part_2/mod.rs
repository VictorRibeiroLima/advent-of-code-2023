use crate::map::{pipe::PipeType, Map, PieceType};

pub fn process(input: &str) -> u32 {
    let map = Map::init(input);
    println!("{}", map);
    let mut count_inside = 0;
    for i in 1..map.map.len() - 1 {
        let line = &map.map[i];
        for j in 1..line.len() - 1 {
            let piece = map.map[i][j].borrow();
            let piece = *piece;
            match piece {
                PieceType::Pipe(pipe) => {
                    if pipe.main_loop_count.is_some() {
                        continue;
                    }
                }
                _ => {}
            };
            let mut count: isize = 0;

            for k in j..line.len() {
                let piece = map.map[i][k].borrow();
                let piece = *piece;
                let pipe = match piece {
                    PieceType::Pipe(pipe) => pipe,
                    _ => continue,
                };
                let pipe_type = pipe.pipe_type;
                match pipe_type {
                    PipeType::Horizontal => continue,
                    PipeType::RightUp => continue,
                    PipeType::LeftUp => continue,
                    _ => {}
                }

                let next_index = match pipe.main_loop_count {
                    Some(index) => index,
                    None => continue,
                };
                let next_index = if next_index == 0 {
                    map.polygon_points.len() - 1
                } else {
                    next_index - 1
                };
                let next = &map.polygon_points[next_index];

                let next_i = next.i as i32;
                let this_i = i as i32;
                let c = next_i - this_i;
                if c > 0 {
                    count += 1;
                } else {
                    count -= 1;
                }
            }
            if count != 0 {
                count_inside += 1;
            }
        }
    }
    return count_inside;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process_4() {
        let input = include_str!("../inputs/4_inner_input.txt");
        let result = process(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_process_8() {
        let input = include_str!("../inputs/8_inner_input.txt");
        let result = process(input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_process_10() {
        let input = include_str!("../inputs/10_inner_input.txt");
        let result = process(input);
        assert_eq!(result, 10);
    }

    #[test]
    fn my_input() {
        let input = include_str!("../inputs/my_input.txt");
        let result = process(input);
        assert_eq!(result, 501);
    }
}
