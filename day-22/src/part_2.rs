use crate::tower::Tower;

pub fn process(input: &str) -> usize {
    let mut tower = Tower::new(input);
    tower.apply_gravity();
    tower.mark_unbreakable();
    let dropped = tower.count_drops();

    dropped
}
