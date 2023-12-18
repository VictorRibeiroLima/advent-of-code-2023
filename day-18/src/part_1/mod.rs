use self::pool::Pool;

mod pool;

pub fn process(input: &str) -> usize {
    let mut pool = Pool::new(input);
    pool.dig();
    pool.count_digged()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 57);
    }
}
