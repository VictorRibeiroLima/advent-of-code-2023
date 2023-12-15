mod part_1;
mod part_2;

fn main() {
    let input = include_str!("./input.txt");
    let result = part_1::process(input);
    println!("part 1: {}", result);
    let result = part_2::process(input);
    println!("part 2: {}", result);
}
