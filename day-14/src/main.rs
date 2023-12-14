mod part_1;
mod part_2;

fn main() {
    let input = include_str!("./inputs/input.txt");
    let result = part_1::process(input);
    println!("{}", result);
    let result = part_2::process(input);
    println!("{}", result);
}
