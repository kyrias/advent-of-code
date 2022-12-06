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

    let mut part1_stacks = stacks.clone();
    let mut part2_stacks = stacks;
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
                let item = part1_stacks[from].pop_back().unwrap();
                part1_stacks[to].push_back(item);
            }

            {
                let from_stack = &mut part2_stacks[from];
                let items: Vec<_> = from_stack.drain(from_stack.len() - number..).collect();
                part2_stacks[to].extend(items);
            }
        });

    let top_of_stacks: String = part1_stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect();
    println!("Part 1: {top_of_stacks}");

    let top_of_stacks: String = part2_stacks
        .iter()
        .map(|stack| stack.back().unwrap())
        .collect();
    println!("Part 2: {top_of_stacks}");
}
