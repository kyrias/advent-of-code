use std::collections::HashSet;

use itertools::repeat_n;

#[derive(Clone, Copy)]
enum Movement {
    Up,
    Down,
    Left,
    Right,
}

impl From<&str> for Movement {
    fn from(c: &str) -> Self {
        match c {
            "U" => Movement::Up,
            "D" => Movement::Down,
            "L" => Movement::Left,
            "R" => Movement::Right,
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    row: isize,
    column: isize,
}

impl Position {
    fn perform_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Up => self.row += 1,
            Movement::Down => self.row -= 1,
            Movement::Left => self.column -= 1,
            Movement::Right => self.column += 1,
        }
    }
}

#[derive(Default, Debug)]
struct State {
    head: Position,
    tail: Position,
}

impl State {
    fn perform_movement(&mut self, movement: Movement) {
        self.head.perform_movement(movement);

        if ((self.tail.row - 1)..=(self.tail.row + 1)).contains(&self.head.row)
            && ((self.tail.column - 1)..=(self.tail.column + 1)).contains(&self.head.column)
        {
            // If the head is still within a one-unit radius the tail doesn't move.
        } else if ((self.tail.row - self.head.row).abs() == 2
            && self.tail.column == self.head.column)
            || ((self.tail.column - self.head.column).abs() == 2 && self.tail.row == self.head.row)
        {
            // The head is exactly two steps directionally away from tail, so move in the same
            // direction.
            self.tail.perform_movement(movement);
        } else if self.head.row < self.tail.row && self.head.column > self.tail.column {
            self.tail.row -= 1;
            self.tail.column += 1;
        } else if self.head.row > self.tail.row && self.head.column > self.tail.column {
            self.tail.row += 1;
            self.tail.column += 1;
        } else if self.head.row > self.tail.row && self.head.column < self.tail.column {
            self.tail.row += 1;
            self.tail.column -= 1;
        } else if self.head.row < self.tail.row && self.head.column < self.tail.column {
            self.tail.row -= 1;
            self.tail.column -= 1;
        }
    }
}

fn main() {
    let movements: Vec<Movement> = std::io::stdin()
        .lines()
        .flat_map(|line| {
            let mut parts = line.as_ref().unwrap().split(' ');
            repeat_n(
                parts.next().unwrap().into(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut state: State = Default::default();
    let mut visited = HashSet::new();
    for movement in movements.iter().copied() {
        state.perform_movement(movement);
        visited.insert(state.tail);
    }
    println!("Unique tail positions: {}", visited.len());
}
