mod part_1;
mod part_2;

fn main() {
    let input = include_str!("./inputs/input.txt");
    println!("Result: {}", part_1::process(input));
    println!("Result: {}", part_2::process(input));
}
