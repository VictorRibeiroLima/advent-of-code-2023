use crate::tower::Tower;

mod tower;

fn main() {
    let input = include_str!("./inputs/input.txt");
    let mut tower = Tower::new(input);
    tower.apply_gravity();
    let destroyable = tower.mark_unbreakable();
    let dropped = tower.count_drops();
    println!("Part 1: {}", destroyable);
    println!("Part 2: {}", dropped);
}
