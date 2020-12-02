use std::io::BufRead;

use itertools::Itertools;

/// Maybe parse one line of stdin into an u64
fn maybe_parse<T>(line: Result<String, T>) -> Option<u64> {
    line.ok().map(|l| l.parse().ok()).flatten()
}

/// Find n-pairs of numbers whose sum is 2020 and return their product.
fn find_pairs(n: usize, numbers: &[u64]) -> Option<u64> {
    for pair in numbers.iter().permutations(n) {
        let sum: u64 = pair.clone().into_iter().sum();
        if sum == 2020 {
            return Some(pair.clone().into_iter().product());
        }
    }

    None
}

fn main() {
    let stdin = std::io::stdin();
    let numbers: Vec<u64> = stdin.lock().lines().filter_map(maybe_parse).collect();

    match find_pairs(2, &numbers) {
        Some(product) => println!("Matching two-pair product: {}", product),
        None => println!("No matching two-pair products found."),
    }

    match find_pairs(3, &numbers) {
        Some(product) => println!("Matching three-pair product: {}", product),
        None => println!("No matching three-pair products found."),
    }
}
