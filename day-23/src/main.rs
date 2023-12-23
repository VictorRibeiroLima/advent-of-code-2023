mod part_1;
mod part_2;

fn main() {
    let input = include_str!("./inputs/input.txt");
    let distance = part_1::process(input);
    println!("Part 1: {}", distance);
    let distance = part_2::process(input);
    println!("Part 2: {}", distance);
}
