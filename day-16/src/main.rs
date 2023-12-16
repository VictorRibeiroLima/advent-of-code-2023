mod grid;
mod part1;
mod part2;

fn main() {
    let input = include_str!("./inputs/input.txt");
    let result = part1::process(input);
    println!("Result: {}", result);
    let result = part2::process(input);
    println!("Result: {}", result);
}
