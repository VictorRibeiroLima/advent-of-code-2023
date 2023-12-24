mod part_1;
fn main() {
    let input = include_str!("./inputs/input.txt");
    let result = part_1::process(input);
    println!("Part 1: {}", result);
}
