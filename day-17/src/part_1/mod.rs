use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    i: usize,
    j: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct State {
    position: Position,
    up_available: u8,
    down_available: u8,
    left_available: u8,
    right_available: u8,
    //The direction used to reach this state
    direction: Direction,
}

pub fn process(input: &str) -> usize {
    let mut matrix = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }
        matrix.push(row);
    }

    let source = Position { i: 0, j: 0 };
    let destination = Position {
        i: matrix.len() - 1,
        j: matrix[0].len() - 1,
    };

    let cost = get_cost(&matrix, source, destination);

    cost
}

fn get_cost(map: &Vec<Vec<usize>>, source: Position, destination: Position) -> usize {
    // Hashmap representing the minimum cost to reach a given state
    let mut costs: HashMap<State, usize> = HashMap::new();
    // Min-heap that will be used to find the state with the lowest cost
    let mut heap: BinaryHeap<Reverse<(usize, State)>> = BinaryHeap::new();

    let initial_state = State {
        position: source,
        up_available: 3,
        down_available: 3,
        left_available: 3,
        right_available: 3,
        direction: Direction::Right,
    };
    costs.insert(initial_state, 0);
    heap.push(Reverse((0, initial_state)));

    while let Some(Reverse((curr_cost, curr_state))) = heap.pop() {
        if curr_state.position == destination {
            return curr_cost;
        }

        let cost_to_curr_state = *costs.get(&curr_state).unwrap_or(&usize::MAX);
        if curr_cost > cost_to_curr_state {
            // We've already found a better path to this state
            continue;
        }

        let neighbors = get_neighbors_states(curr_state, map);

        for neighbor in neighbors {
            let next_pos = neighbor.position;
            let node_cost = map[next_pos.i][next_pos.j];
            let next_cost = curr_cost + node_cost;
            if next_cost < *costs.get(&neighbor).unwrap_or(&usize::MAX) {
                heap.push(Reverse((next_cost, neighbor)));
                costs.insert(neighbor, next_cost);
            }
        }
    }
    usize::MAX
}

//Get the neighbors of a given state
fn get_neighbors_states(state: State, map: &Vec<Vec<usize>>) -> Vec<State> {
    let mut neighbors = Vec::new();
    let position = state.position;
    let used_direction = state.direction;

    //Check Up
    if state.up_available > 0 && used_direction != Direction::Down && position.i > 0 {
        let up_position = Position {
            i: position.i - 1,
            j: position.j,
        };

        //When a state when decrease the number of available moves for the direction used to reach it,and reset the other directions
        let up_state = State {
            position: up_position,
            up_available: state.up_available - 1,
            down_available: 3,
            left_available: 3,
            right_available: 3,
            direction: Direction::Up,
        };
        neighbors.push(up_state);
    }

    //Check Down
    if state.down_available > 0 && used_direction != Direction::Up && position.i < map.len() - 1 {
        let down_position = Position {
            i: position.i + 1,
            j: position.j,
        };
        let down_state = State {
            position: down_position,
            up_available: 3,
            down_available: state.down_available - 1,
            left_available: 3,
            right_available: 3,
            direction: Direction::Down,
        };
        neighbors.push(down_state);
    }

    //Check Left
    if state.left_available > 0 && used_direction != Direction::Right && position.j > 0 {
        let left_position = Position {
            i: position.i,
            j: position.j - 1,
        };
        let left_state = State {
            position: left_position,
            up_available: 3,
            down_available: 3,
            left_available: state.left_available - 1,
            right_available: 3,
            direction: Direction::Left,
        };
        neighbors.push(left_state);
    }

    //Check Right
    if state.right_available > 0
        && used_direction != Direction::Left
        && position.j < map[0].len() - 1
    {
        let right_position = Position {
            i: position.i,
            j: position.j + 1,
        };
        let right_state = State {
            position: right_position,
            up_available: 3,
            down_available: 3,
            left_available: 3,
            right_available: state.right_available - 1,
            direction: Direction::Right,
        };
        neighbors.push(right_state);
    }

    neighbors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_neighbors() {
        let matrix = vec![vec![2, 4, 1], vec![3, 2, 1], vec![3, 2, 5]];
        let initial_state = State {
            position: Position { i: 0, j: 0 },
            up_available: 3,
            down_available: 3,
            left_available: 3,
            right_available: 3,
            direction: Direction::Right,
        };
        let neighbors = get_neighbors_states(initial_state, &matrix);
        assert_eq!(neighbors.len(), 2);
    }

    #[test]
    fn test_process() {
        /*
         2 4 1 3 4
         3 2 1 5 4
         3 2 5 5 2
        */
        let input = "24134\n32154\n32552";
        let cost = process(input);
        assert_eq!(cost, 17);
    }

    #[test]
    fn test_process2() {
        let input = include_str!("../inputs/test.txt");
        let cost = process(input);
        assert_eq!(cost, 102);
    }
}
