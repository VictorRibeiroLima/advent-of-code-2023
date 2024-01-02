use crate::tower::Tower;

pub fn process(input: &str) -> usize {
    let mut tower = Tower::new(input);
    tower.apply_gravity();
    let destroyable = tower.mark_unbreakable();
    destroyable
}
