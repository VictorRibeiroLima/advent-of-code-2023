use super::line::{Line, Point};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum LimitType {
    Min(usize),
    Max(usize),
    Never,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct HailStone {
    x: i64,
    y: i64,
    z: i64,
    x_change_rate: i64,
    y_change_rate: i64,
    z_change_rate: i64,
    x_reaches_limit: LimitType,
    y_reaches_limit: LimitType,
    z_reaches_limit: LimitType,
    min: f64,
    max: f64,
}

impl HailStone {
    pub fn new(input: &str, limits: (i64, i64)) -> Self {
        let (cords, change) = input.split_at(input.find('@').unwrap());
        let cords = cords.trim();
        let change = change.trim_matches(|c| c == '@' || c == ' ');
        let cords = Self::parse_cords(cords);
        let change = Self::parse_cords(change);
        let x_change_rate = change.0;
        let y_change_rate = change.1;
        let z_change_rate = change.2;
        let x_reaches_limit =
            check_in_how_many_steps_limit_is_reached(cords.0, x_change_rate, limits);
        let y_reaches_limit =
            check_in_how_many_steps_limit_is_reached(cords.1, y_change_rate, limits);
        let z_reaches_limit =
            check_in_how_many_steps_limit_is_reached(cords.2, z_change_rate, limits);

        Self {
            x: cords.0,
            y: cords.1,
            z: cords.2,
            x_change_rate,
            y_change_rate,
            z_change_rate,
            x_reaches_limit,
            y_reaches_limit,
            z_reaches_limit,
            min: limits.0 as f64,
            max: limits.1 as f64,
        }
    }

    pub fn paths_intersect(&self, other: &Self) -> Option<Point> {
        let x_steps = match self.x_reaches_limit {
            LimitType::Min(steps) => steps,
            LimitType::Max(steps) => steps,
            LimitType::Never => return None,
        };
        let y_steps = match self.y_reaches_limit {
            LimitType::Min(steps) => steps,
            LimitType::Max(steps) => steps,
            LimitType::Never => return None,
        };
        let min = std::cmp::min(x_steps, y_steps);

        let other_x_steps = match other.x_reaches_limit {
            LimitType::Min(steps) => steps,
            LimitType::Max(steps) => steps,
            LimitType::Never => return None,
        };
        let other_y_steps = match other.y_reaches_limit {
            LimitType::Min(steps) => steps,
            LimitType::Max(steps) => steps,
            LimitType::Never => return None,
        };

        let x = self.x + (self.x_change_rate * x_steps as i64);
        let y = self.y + (self.y_change_rate * y_steps as i64);

        let other_x = other.x + (other.x_change_rate * other_x_steps as i64);
        let other_y = other.y + (other.y_change_rate * other_y_steps as i64);

        let min_x = std::cmp::min(x, self.x) as f64;
        let max_x = std::cmp::max(x, self.x) as f64;
        let min_y = std::cmp::min(y, self.y) as f64;
        let max_y = std::cmp::max(y, self.y) as f64;

        let other_min_x = std::cmp::min(other_x, other.x) as f64;
        let other_max_x = std::cmp::max(other_x, other.x) as f64;
        let other_min_y = std::cmp::min(other_y, other.y) as f64;
        let other_max_y = std::cmp::max(other_y, other.y) as f64;

        let other_min = std::cmp::min(other_x_steps, other_y_steps);

        let steps = std::cmp::min(min, other_min);

        let x_steps: i64 = self.x + self.x_change_rate * steps as i64;
        let y_steps = self.y + self.y_change_rate * steps as i64;

        let other_x_steps = other.x + other.x_change_rate * steps as i64;
        let other_y_steps = other.y + other.y_change_rate * steps as i64;

        let self_point1 = Point::new(self.x, self.y);
        let self_point2 = Point::new(x_steps, y_steps);
        let self_line = Line::new(self_point1, self_point2);

        let other_point1 = Point::new(other.x, other.y);
        let other_point2 = Point::new(other_x_steps, other_y_steps);
        let other_line = Line::new(other_point1, other_point2);

        let intersection = self_line.intersection(&other_line);
        match intersection {
            Some(point) => {
                //Check if the intersection point is within the range of the stone
                if point.x >= self.min
                    && point.x <= self.max
                    && point.y >= self.min
                    && point.y <= self.max
                {
                    let is_between_xs = point.x >= min_x
                        && point.x <= max_x
                        && point.x >= other_min_x
                        && point.x <= other_max_x;

                    let is_between_ys = point.y >= min_y
                        && point.y <= max_y
                        && point.y >= other_min_y
                        && point.y <= other_max_y;

                    if is_between_xs && is_between_ys {
                        return Some(point);
                    }

                    return None;
                }
                None
            }
            None => None,
        }
    }

    fn parse_cords(cords: &str) -> (i64, i64, i64) {
        let mut cords = cords.split(',');
        let x = cords.next().unwrap().trim().parse().unwrap();
        let y = cords.next().unwrap().trim().parse().unwrap();
        let z = cords.next().unwrap().trim().parse().unwrap();
        (x, y, z)
    }
}

fn check_in_how_many_steps_limit_is_reached(
    number: i64,
    change_rate: i64,
    limits: (i64, i64),
) -> LimitType {
    let max_limit = limits.1;
    let min_limit = limits.0;
    if change_rate == 0 {
        return LimitType::Never;
    }
    if change_rate > 0 {
        if number > max_limit {
            return LimitType::Never;
        } else if number == max_limit {
            return LimitType::Max(0);
        }
        let steps = (max_limit - number) / change_rate;
        let remainder = (max_limit - number) % change_rate;
        let steps = if remainder < 0 { steps - 1 } else { steps };
        LimitType::Max(steps as usize)
    } else {
        if number < min_limit {
            return LimitType::Never;
        } else if number == min_limit {
            return LimitType::Min(0);
        }
        let steps = (number - min_limit) / change_rate;
        let remainder = (number - min_limit) % change_rate;
        let steps = if remainder < 0 { steps - 1 } else { steps };
        LimitType::Min(steps.abs() as usize)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::part_1::{MAX, MIN};

    #[test]
    fn test_path_intersection_edge_case() {
        let input1 = "19, 13, 30 @ -2, 1, -2";
        let input2 = "12, 31, 28 @ -1, -2, -1";
        let stone1 = HailStone::new(input1, (7, 27));
        let stone2 = HailStone::new(input2, (7, 27));
        let intersects = stone1.paths_intersect(&stone2);
        assert!(intersects.is_none());
    }

    #[test]
    fn test_path_intersection_edge_case2() {
        let input1 = "19, 13, 30 @ -2, 1, -2";
        let input2 = "20, 25, 34 @ -2, -2, -4";
        let stone1 = HailStone::new(input1, (7, 27));
        let stone2 = HailStone::new(input2, (7, 27));
        let intersects = stone1.paths_intersect(&stone2);
        assert!(intersects.is_some());
    }

    #[test]
    fn test_path_intersection_edge_case3() {
        let input1 = "19, 13, 30 @ -2, 1, -2";
        let input2 = "20, 19, 15 @ 1, -5, -3";
        let stone1 = HailStone::new(input1, (7, 27));
        let stone2 = HailStone::new(input2, (7, 27));
        let intersects = stone1.paths_intersect(&stone2);
        assert!(intersects.is_none());
    }

    #[test]
    fn test_new() {
        let input = "20, 13, 30 @ -2, 1, -2";
        let expected_cords = (20, 13, 30);
        let expected_change = (-2, 1, -2);
        let stone = HailStone::new(input, (0, 100));
        assert_eq!(stone.x, expected_cords.0);
        assert_eq!(stone.y, expected_cords.1);
        assert_eq!(stone.z, expected_cords.2);
        assert_eq!(stone.x_change_rate, expected_change.0);
        assert_eq!(stone.y_change_rate, expected_change.1);
        assert_eq!(stone.z_change_rate, expected_change.2);
        assert_eq!(stone.x_reaches_limit, LimitType::Min(10));
        assert_eq!(stone.y_reaches_limit, LimitType::Max(87));
        assert_eq!(stone.z_reaches_limit, LimitType::Min(15));
    }

    #[test]
    fn test_new2() {
        let input = "19, 13, 30 @ -2, 1, -2";
        let expected_cords: (i64, i64, i64) = (19, 13, 30);
        let expected_change = (-2, 1, -2);
        let stone = HailStone::new(input, (0, 100));
        assert_eq!(stone.x, expected_cords.0);
        assert_eq!(stone.y, expected_cords.1);
        assert_eq!(stone.z, expected_cords.2);
        assert_eq!(stone.x_change_rate, expected_change.0);
        assert_eq!(stone.y_change_rate, expected_change.1);
        assert_eq!(stone.z_change_rate, expected_change.2);
        assert_eq!(stone.x_reaches_limit, LimitType::Min(9));
        assert_eq!(stone.y_reaches_limit, LimitType::Max(87));
        assert_eq!(stone.z_reaches_limit, LimitType::Min(15));
    }

    #[test]
    fn test_new3() {
        let input = "19, 13, 30 @ -2, 10, -2";
        let expected_cords: (i64, i64, i64) = (19, 13, 30);
        let expected_change = (-2, 10, -2);
        let stone = HailStone::new(input, (0, 100));
        assert_eq!(stone.x, expected_cords.0);
        assert_eq!(stone.y, expected_cords.1);
        assert_eq!(stone.z, expected_cords.2);
        assert_eq!(stone.x_change_rate, expected_change.0);
        assert_eq!(stone.y_change_rate, expected_change.1);
        assert_eq!(stone.z_change_rate, expected_change.2);
        assert_eq!(stone.x_reaches_limit, LimitType::Min(9));
        assert_eq!(stone.y_reaches_limit, LimitType::Max(8));
        assert_eq!(stone.z_reaches_limit, LimitType::Min(15));
    }

    #[test]
    fn test_new_bigger() {
        let input = "346929738756520, 180308062329517, 348158644025623 @ 6, -5, -22";
        let expected_cords = (346929738756520, 180308062329517, 348158644025623);
        let expected_change = (6, -5, -22);
        let stone = HailStone::new(input, (MIN, MAX));
        assert_eq!(stone.x, expected_cords.0);
        assert_eq!(stone.y, expected_cords.1);
        assert_eq!(stone.z, expected_cords.2);
        assert_eq!(stone.x_change_rate, expected_change.0);
        assert_eq!(stone.y_change_rate, expected_change.1);
        assert_eq!(stone.z_change_rate, expected_change.2);
        assert_eq!(stone.x_reaches_limit, LimitType::Max(8845043540580));
        assert_eq!(stone.y_reaches_limit, LimitType::Never);
        assert_eq!(stone.z_reaches_limit, LimitType::Min(6734483819346));
    }
}
