use std::io::BufRead;

use itertools::Itertools;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut numbers = Vec::new();
    for line in std::io::stdin().lock().lines() {
        numbers.push(line?.parse::<usize>()?);
    }

    let part1 = numbers
        .iter()
        .tuple_windows()
        .fold(0, |acc, (previous, current)| {
            if current > previous {
                acc + 1
            } else {
                acc
            }
        });

    println!("Part 1: {}", part1);


    let part2 = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |count, (previous, current)| {
            if current > previous {
                count + 1
            } else {
                count
            }
        });

    println!("Part 2: {}", part2);

    Ok(())
}
