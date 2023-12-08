mod part_1;
mod part_2;

fn main() {
    let input = include_str!("./inputs/my_input.txt");
    let result = part_1::process(input);
    println!("Result pt 1: {}", result);
    let result = part_2::process(input);
    println!("Result pt 2: {}", result);
}
