mod part_1;
mod part_2;
fn main() {
    let input = include_str!("./inputs/my_input.txt");
    let part1 = part_1::process(input);
    println!("Part 1: {}", part1);
    let part2 = part_2::process(input);
    println!("Part 2: {}", part2);
}
