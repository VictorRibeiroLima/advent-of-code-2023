pub fn process(input: &str) -> i64 {
    let mut points = Vec::new();
    let mut point = (0, 0);
    let mut perimeter = 0;
    for line in input.lines() {
        let hex = line.split(" ").nth(2).unwrap();
        let hex = hex.as_bytes();
        let length = &hex[2..=6];
        let length = std::str::from_utf8(length).unwrap();
        let length = i64::from_str_radix(length, 16).unwrap();
        let direction = hex[7] as char;
        match direction {
            '0' => point.1 += length as i64,
            '1' => point.0 += length as i64,
            '2' => point.1 -= length as i64,
            '3' => point.0 -= length as i64,
            _ => panic!("Invalid direction"),
        }
        perimeter += length;
        points.push(point);
    }
    let area = shoelace_formula(&points);
    let result = (perimeter / 2) + area + 1;

    result
}

fn shoelace_formula(points: &Vec<(i64, i64)>) -> i64 {
    let mut sum = 0;
    for i in 0..points.len() {
        let j = (i + 1) % points.len();
        let x_i = points[i].0;
        let y_i = points[i].1;
        let x_j = points[j].0;
        let y_j = points[j].1;
        sum += x_i * y_j - x_j * y_i;
    }
    sum.abs() / 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = include_str!("../inputs/test.txt");
        let result = process(input);
        assert_eq!(result, 952408144115);
    }

    #[test]
    fn test_process2() {
        let input = include_str!("../inputs/input.txt");
        let result = process(input);
        assert_eq!(result, 173152345887206);
    }
}
