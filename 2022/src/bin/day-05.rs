use std::collections::VecDeque;

use odds::string::StrChunksWindows;

fn main() {
    let mut stacks = Vec::new();
    let mut lines = std::io::stdin().lines().map(|line| line.unwrap());

    // Because `take_while` needs to look at the last element this will also end up skipping the
    // header under the stack diagram.
    lines
        .by_ref()
        .take_while(|line| line.contains('['))
        .for_each(|line| {
            line.char_chunks(4).enumerate().for_each(|(idx, item)| {
                while stacks.len() < idx + 1 {
                    stacks.push(VecDeque::new());
                }
                let stack = &mut stacks[idx];
                if !item.trim().is_empty() {
                    stack.push_front(item.chars().nth(1).unwrap());
                }
            })
        });

    // Skip empty line
    lines.next();

    lines
        .map(|m| {
            let mut fields = m.split(' ');
            let number: usize = fields.nth(1).unwrap().parse().unwrap();
            let from: usize = fields.nth(1).unwrap().parse().unwrap();
            let to: usize = fields.nth(1).unwrap().parse().unwrap();
            (number, from - 1, to - 1)
        })
        .for_each(|(number, from, to)| {
            for _ in 0..number {
                let item = stacks[from].pop_back().unwrap();
                stacks[to].push_back(item);
            }
        });

    let top_of_stacks: String = stacks.iter().map(|stack| stack.back().unwrap()).collect();
    println!("{top_of_stacks}");
}
