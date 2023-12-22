type Cord = (usize, usize, usize);

#[derive(Debug, Clone, Copy, Hash)]
pub struct Brick {
    pub first_end: Cord,
    pub second_end: Cord,
}

impl PartialEq for Brick {
    //A brick can be flipped, so we need to check both orientations
    fn eq(&self, other: &Self) -> bool {
        if self.first_end == other.first_end && self.second_end == other.second_end {
            return true;
        }
        if self.first_end == other.second_end && self.second_end == other.first_end {
            return true;
        }
        false
    }
}

impl Eq for Brick {}

impl PartialOrd for Brick {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let z_order = self.first_end.2.partial_cmp(&other.first_end.2);
        if z_order != Some(std::cmp::Ordering::Equal) {
            return z_order;
        }

        let y_order = self.first_end.1.partial_cmp(&other.first_end.1);
        if y_order != Some(std::cmp::Ordering::Equal) {
            return y_order;
        }

        self.first_end.0.partial_cmp(&other.first_end.0)
    }
}

impl Ord for Brick {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl Brick {
    pub fn new(input: &str) -> Brick {
        let mut iter = input.split("~");
        let first_end = parse_cord(iter.next().unwrap());
        let second_end = parse_cord(iter.next().unwrap());
        Brick {
            first_end: first_end,
            second_end: second_end,
        }
    }
}

fn parse_cord(input: &str) -> Cord {
    let mut iter = input.split(",");
    let first = iter.next().unwrap().parse::<usize>().unwrap();
    let second = iter.next().unwrap().parse::<usize>().unwrap();
    let third = iter.next().unwrap().parse::<usize>().unwrap();
    (first, second, third)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_parse_cord() {
        let input = "1,2,3";
        let expected = (1, 2, 3);
        let actual = parse_cord(input);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_brick_new() {
        let input = "1,2,3~4,5,6";
        let expected = Brick {
            first_end: (1, 2, 3),
            second_end: (4, 5, 6),
        };
        let actual = Brick::new(input);
        assert_eq!(expected.first_end, actual.first_end);
        assert_eq!(expected.second_end, actual.second_end);
    }
}
