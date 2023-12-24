#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x: x as f64,
            y: y as f64,
        }
    }
}

pub struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    pub fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    pub fn intersection(&self, l2: &Self) -> Option<Point> {
        let m1 = self.slope();
        let m2 = l2.slope();

        if m1 == m2 {
            None // lines are parallel, no intersection
        } else {
            let b1 = self.y_intercept();
            let b2 = l2.y_intercept();

            let x = (b2 - b1) / (m1 - m2);
            let y = m1 * x + b1;

            Some(Point { x, y })
        }
    }

    fn slope(&self) -> f64 {
        (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x)
    }

    fn y_intercept(&self) -> f64 {
        self.p1.y - self.slope() * self.p1.x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_intersection_edge() {
        let l1 = Line {
            p1: Point { x: 0.0, y: 0.0 },
            p2: Point { x: 1.0, y: 1.0 },
        };
        let l2 = Line {
            p1: Point { x: 0.0, y: 1.0 },
            p2: Point { x: 1.0, y: 0.0 },
        };

        let intersect = l1.intersection(&l2).unwrap();
        assert_eq!(intersect.x, 0.5);
        assert_eq!(intersect.y, 0.5);
    }
}
