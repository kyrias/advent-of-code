use std::collections::HashSet;

fn priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 96,
        'A'..='Z' => item as usize - 38,
        _ => panic!("Unexpected item"),
    }
}

fn main() {
    let sum: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let half = line.len() / 2;

            let first: HashSet<char> = line[..half].chars().collect();
            let second: HashSet<char> = line[half..].chars().collect();
            first.intersection(&second).copied().map(priority).sum::<usize>()
        })
        .sum();
    println!("Sum: {}", sum);
}
