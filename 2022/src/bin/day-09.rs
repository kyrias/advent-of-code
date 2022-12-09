use std::collections::HashSet;

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
    knots: [Position; 10],
}

impl State {
    fn perform_movement(&mut self, movement: Movement) {
        self.knots[0].perform_movement(movement);

        let mut iter = self.knots.iter_mut();
        let mut head = iter.next().unwrap();
        for tail in iter {
            if ((tail.row - 1)..=(tail.row + 1)).contains(&head.row)
                && ((tail.column - 1)..=(tail.column + 1)).contains(&head.column)
            {
                // If the head is still within a one-unit radius the tail doesn't move.
            } else if ((tail.row - head.row).abs() == 2 && tail.column == head.column)
                || ((tail.column - head.column).abs() == 2 && tail.row == head.row)
            {
                // The head is exactly two steps directionally away from tail, so move in the same
                // direction.
                if tail.row == head.row - 2 {
                    tail.row += 1;
                } else if tail.row == head.row + 2 {
                    tail.row -= 1;
                } else if tail.column == head.column - 2 {
                    tail.column += 1;
                } else if tail.column == head.column + 2 {
                    tail.column -= 1;
                } else {
                    unreachable!();
                }
            } else if head.row < tail.row && head.column > tail.column {
                tail.row -= 1;
                tail.column += 1;
            } else if head.row > tail.row && head.column > tail.column {
                tail.row += 1;
                tail.column += 1;
            } else if head.row > tail.row && head.column < tail.column {
                tail.row += 1;
                tail.column -= 1;
            } else if head.row < tail.row && head.column < tail.column {
                tail.row -= 1;
                tail.column -= 1;
            }
            head = tail;
        }
    }
}

fn main() {
    let movements: Vec<(Movement, usize)> = std::io::stdin()
        .lines()
        .map(|line| {
            let mut parts = line.as_ref().unwrap().split(' ');
            (
                parts.next().unwrap().into(),
                parts.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let mut state: State = Default::default();

    let mut visited_part1 = HashSet::new();
    let mut visited_part2 = HashSet::new();
    for (movement, number) in movements.iter().copied() {
        for _ in 0..number {
            state.perform_movement(movement);
            visited_part1.insert(state.knots[1]);
            visited_part2.insert(state.knots[9]);
        }
    }
    println!("Unique tail positions: {}", visited_part1.len());
    println!("Unique tail positions: {}", visited_part2.len());
}
