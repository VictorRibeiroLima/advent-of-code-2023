#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add,
    Remove,
}

pub fn process(input: &str) -> usize {
    let mut result = 0;
    let mut boxes: [Vec<String>; 256] = vec![Vec::new(); 256].try_into().unwrap();
    for line in input.lines() {
        for part in line.split(',') {
            let mut value = 0;
            let mut label = String::new();
            let mut operation = Operation::Remove;
            for c in part.chars() {
                if c == '-' {
                    break;
                }
                if c == '=' {
                    operation = Operation::Add;
                    continue;
                }
                label.push(c);
                if operation == Operation::Remove {
                    value += (c as u8) as usize;
                    value *= 17;
                    value %= 256;
                }
            }
            match operation {
                Operation::Add => {
                    add(&mut boxes, label, value);
                }
                Operation::Remove => {
                    remove(&mut boxes, label, value);
                }
            }
        }
    }

    for (i, box_) in boxes.into_iter().enumerate() {
        let box_value = i + 1;
        result += calculate_box_value(box_, box_value);
    }
    result
}

fn calculate_box_value(box_: Vec<String>, box_value: usize) -> usize {
    let mut result = 0;
    for (i, value) in box_.into_iter().enumerate() {
        let len = value.len();
        let (_, value) = value.split_at(len - 1);
        let slot = i + 1;
        let value = match value.parse::<usize>() {
            Ok(value) => value,
            Err(_) => {
                panic!("{} is not a number", value)
            }
        };
        result = result + (box_value * slot * value);
    }
    result
}

fn add(boxes: &mut [Vec<String>; 256], new_value: String, index: usize) {
    let len = new_value.len();
    let (label, _) = new_value.split_at(len - 1);
    let box_ = &mut boxes[index];
    let mut index = None;
    for (i, value) in box_.iter().enumerate() {
        let len = value.len();
        let (value_label, _) = value.split_at(len - 1);
        if value_label == label {
            index = Some(i);
            break;
        }
    }

    match index {
        Some(index) => {
            let value = &mut box_[index];
            *value = new_value;
        }
        None => {
            box_.push(new_value);
        }
    }
}

fn remove(boxes: &mut [Vec<String>; 256], label: String, index: usize) {
    let box_ = &mut boxes[index];
    let mut index = None;
    for (i, value) in box_.iter().enumerate() {
        let len = value.len();
        let (value_label, _) = value.split_at(len - 1);
        if value_label == label {
            index = Some(i);
            break;
        }
    }
    match index {
        Some(index) => {
            box_.remove(index);
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input), 145);
    }

    #[test]
    fn test_vec() {
        let mut v = vec![1, 2, 3, 4, 5];
        v.remove(2);
        assert_eq!(v, vec![1, 2, 4, 5]);
    }

    #[test]
    fn test_str() {
        let s = String::from("rn1");
        let (label, _) = s.split_at(2);
        assert_eq!(label, "rn");
    }
}
