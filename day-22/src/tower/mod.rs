use std::collections::{HashMap, HashSet};

use self::brick::Brick;

mod brick;

#[derive(Clone)]
pub struct Tower {
    tower: Vec<Vec<Vec<Option<Brick>>>>,
    bricks: Vec<Brick>,
    unbreakable: Vec<usize>,
}

impl Tower {
    pub fn new(input: &str) -> Tower {
        let mut brick_queue = Vec::new();
        let mut layer = 0;
        let mut column = 0;
        let mut row = 0;
        for line in input.lines() {
            let brick = Brick::new(line);
            if brick.first_end.2 > layer {
                layer = brick.first_end.2;
            }
            if brick.first_end.1 > column {
                column = brick.first_end.1;
            }
            if brick.first_end.0 > row {
                row = brick.first_end.0;
            }
            if brick.second_end.2 > layer {
                layer = brick.second_end.2;
            }
            if brick.second_end.1 > column {
                column = brick.second_end.1;
            }
            if brick.second_end.0 > row {
                row = brick.second_end.0;
            }
            brick_queue.push(brick);
        }
        let mut base = create_cube(layer, row, column);
        let bricks = brick_queue.clone();
        for brick in brick_queue {
            insert_in_correct_place(brick, &mut base);
        }

        Tower {
            tower: base,
            bricks,
            unbreakable: Vec::new(),
        }
    }

    //Returns the number of bricks that were dropped
    pub fn apply_gravity(&mut self) -> usize {
        let mut count = 0;
        let mut bricks = std::mem::replace(&mut self.bricks, Vec::new());
        bricks.sort();
        for mut brick in bricks {
            let mut dropped = false;
            while !self.was_brick_bellow(&brick) {
                dropped = true;
                self.move_brick_down(&mut brick);
            }
            if dropped {
                count += 1;
            }
            self.bricks.push(brick);
        }
        self.bricks.sort();
        count
    }

    //Returns the number of bricks that can be destroyed
    pub fn mark_unbreakable(&mut self) -> usize {
        let mut count = 0;
        std::mem::swap(&mut self.unbreakable, &mut Vec::new());
        for (i, brick) in self.bricks.iter().enumerate() {
            let bricks_above = self.get_bricks_above(brick);
            if bricks_above.len() == 0 {
                count += 1;
            } else {
                let mut count_bricks: HashMap<Brick, usize> = HashMap::new();
                for brick_above in bricks_above {
                    count_bricks.entry(brick_above).or_insert(0);
                }
                let same_line_bricks = self.get_same_line_bricks(brick);
                for same_line_brick in same_line_bricks {
                    let same_line_bricks_above = self.get_bricks_above(&same_line_brick);
                    for same_line_brick_above in same_line_bricks_above {
                        if let Some(count) = count_bricks.get_mut(&same_line_brick_above) {
                            *count += 1;
                        }
                    }
                }
                let mut destroyable = true;
                for (_, count) in count_bricks {
                    if count == 0 {
                        destroyable = false;
                        break;
                    }
                }
                if destroyable {
                    count += 1;
                } else {
                    self.unbreakable.push(i);
                }
            }
        }
        count
    }

    pub fn count_drops(&self) -> usize {
        let mut count = 0;
        for i in 0..self.unbreakable.len() {
            let mut tower = self.clone();
            let removed = self.unbreakable[i];
            tower.remove_brick(removed);
            let dropped = tower.apply_gravity();
            count += dropped;
        }
        count
    }

    fn remove_brick(&mut self, i: usize) {
        let brick = self.bricks.remove(i);
        self.remove_brick_from_tower(brick);
    }

    fn remove_brick_from_tower(&mut self, brick: Brick) {
        let mut z = brick.first_end.2;
        let mut y = brick.first_end.1;
        let mut x = brick.first_end.0;
        let z_target = brick.second_end.2;
        let y_target = brick.second_end.1;
        let x_target = brick.second_end.0;
        while z <= z_target {
            while x <= x_target {
                while y <= y_target {
                    self.tower[z][x][y] = None;
                    y += 1;
                }
                y = brick.first_end.1;
                x += 1;
            }
            x = brick.first_end.0;
            z += 1;
        }
    }

    // "z" is the line
    fn get_same_line_bricks(&self, brick: &Brick) -> Vec<Brick> {
        let z = brick.second_end.2;
        let x_len = self.tower[0].len();
        let y_len = self.tower[0][0].len();

        let mut bricks = HashSet::new();
        for x in 0..x_len {
            for y in 0..y_len {
                if let Some(b) = self.tower[z][x][y] {
                    if b != *brick && b.second_end.2 == z {
                        bricks.insert(b);
                    }
                }
            }
        }

        bricks.into_iter().collect()
    }

    //Above is going down the tower
    fn get_bricks_above(&self, brick: &Brick) -> HashSet<Brick> {
        let z = brick.second_end.2;
        let mut y = brick.first_end.1;
        let mut x = brick.first_end.0;
        let y_target = brick.second_end.1;
        let x_target = brick.second_end.0;
        let mut bricks = HashSet::new();
        if z == self.tower.len() - 1 {
            return bricks.into_iter().collect();
        }

        while x <= x_target {
            while y <= y_target {
                if let Some(b) = self.tower[z + 1][x][y] {
                    bricks.insert(b);
                }
                y += 1;
            }
            y = brick.first_end.1;
            x += 1;
        }

        let bricks: HashSet<Brick> = bricks.into_iter().collect();
        bricks
    }

    //Bellow is going up the tower
    fn was_brick_bellow(&self, brick: &Brick) -> bool {
        let z = brick.first_end.2;
        let mut y = brick.first_end.1;
        let mut x = brick.first_end.0;
        let y_target = brick.second_end.1;
        let x_target = brick.second_end.0;
        if z == 0 {
            return true;
        }

        while x <= x_target {
            while y <= y_target {
                if let Some(_) = self.tower[z - 1][x][y] {
                    return true;
                }
                y += 1;
            }
            y = brick.first_end.1;
            x += 1;
        }

        false
    }

    fn move_brick_down(&mut self, brick: &mut Brick) {
        self.remove_brick_from_tower(*brick);

        //Insert brick in new place
        brick.first_end.2 -= 1;
        brick.second_end.2 -= 1;

        let mut z = brick.first_end.2;
        let mut y = brick.first_end.1;
        let mut x = brick.first_end.0;
        let z_target = brick.second_end.2;
        let y_target = brick.second_end.1;
        let x_target = brick.second_end.0;
        while z <= z_target {
            while x <= x_target {
                while y <= y_target {
                    if self.tower[z][x][y].is_some() {
                        panic!("Overlapping bricks");
                    }
                    self.tower[z][x][y] = Some(*brick);
                    y += 1;
                }
                y = brick.first_end.1;
                x += 1;
            }
            x = brick.first_end.0;
            z += 1;
        }
    }
}

fn insert_in_correct_place(brick: Brick, tower: &mut Vec<Vec<Vec<Option<Brick>>>>) {
    let mut z = brick.first_end.2;
    let mut y = brick.first_end.1;
    let mut x = brick.first_end.0;
    let z_target = brick.second_end.2;
    let y_target = brick.second_end.1;
    let x_target = brick.second_end.0;
    while z <= z_target {
        while x <= x_target {
            while y <= y_target {
                tower[z][x][y] = Some(brick);
                y += 1;
            }
            y = brick.first_end.1;
            x += 1;
        }
        x = brick.first_end.0;
        z += 1;
    }
}

// Z is the line, x is the depth, y is the column
fn create_cube(z: usize, mut x: usize, mut y: usize) -> Vec<Vec<Vec<Option<Brick>>>> {
    if x > y {
        y = x;
    } else if y > x {
        x = y;
    }
    let mut cube = Vec::new();
    for _ in 0..z + 1 {
        let mut layer = Vec::new();
        for _ in 0..x + 1 {
            let mut row = Vec::new();
            for _ in 0..y + 1 {
                row.push(None);
            }
            layer.push(row);
        }
        cube.push(layer);
    }
    cube
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_insert_in_correct_place() {
        let input = "1,0,1~1,2,1";
        let brick = Brick::new(input);

        let mut column = 0;
        let mut row = 0;

        if brick.first_end.1 > column {
            column = brick.first_end.1;
        }
        if brick.first_end.0 > row {
            row = brick.first_end.0;
        }
        if brick.second_end.1 > column {
            column = brick.second_end.1;
        }
        if brick.second_end.0 > row {
            row = brick.second_end.0;
        }

        let mut base = create_cube(9, row, column);

        insert_in_correct_place(brick, &mut base);
    }

    #[test]
    fn test_gravity() {
        let input = include_str!("../inputs/test.txt");
        let mut tower = Tower::new(input);

        tower.apply_gravity();
    }

    #[test]
    fn test_gravity3() {
        let input = include_str!("../inputs/input.txt");
        let mut tower = Tower::new(input);

        tower.apply_gravity();
    }

    #[test]
    fn test_same_line() {
        let input = include_str!("../inputs/test.txt");
        let mut tower = Tower::new(input);
        tower.apply_gravity();
        let first_brick = tower.bricks[0];
        let bricks = tower.get_same_line_bricks(&first_brick);
        assert!(bricks.len() == 0);
        let second_brick = tower.bricks[1];
        let bricks = tower.get_same_line_bricks(&second_brick);
        assert_eq!(bricks.len(), 1);
    }

    #[test]
    fn test_destroyable() {
        let input = include_str!("../inputs/test.txt");
        let mut tower = Tower::new(input);
        assert_eq!(tower.bricks.len(), 7);
        tower.apply_gravity();
        let destroyable = tower.mark_unbreakable();
        assert_eq!(destroyable, 5);
        assert_eq!(tower.unbreakable.len(), 2);
    }

    #[test]
    fn test_destroyable2() {
        let input = include_str!("../inputs/input.txt");
        let mut tower = Tower::new(input);
        tower.apply_gravity();
        assert_eq!(tower.bricks.len(), 1325);
        let destroyable = tower.mark_unbreakable();
        assert_eq!(destroyable, 454);
        assert_eq!(tower.unbreakable.len(), 871);
    }
}
