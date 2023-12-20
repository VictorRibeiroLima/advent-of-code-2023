use crate::machine;

const TOTAL_CYCLES: u32 = 1_000;

pub fn process(input: &str) -> u64 {
    let mut machine = machine::Machine::new(input);

    for _ in 0..TOTAL_CYCLES {
        machine.push_button();
    }
    machine.force()
}

#[cfg(test)]
mod test {
    #[test]
    fn test_process() {
        let input = include_str!("./input.txt");
        assert_eq!(super::process(input), 898731036);
    }
}
