use std::collections::HashSet;

fn priority(item: char) -> usize {
    match item {
        'a'..='z' => item as usize - 96,
        'A'..='Z' => item as usize - 38,
        _ => panic!("Unexpected item"),
    }
}

fn main() {
    let (total, intersections) = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let half = line.len() / 2;

            let first: HashSet<char> = line[..half].chars().collect();
            let second: HashSet<char> = line[half..].chars().collect();

            (
                first
                    .intersection(&second)
                    .copied()
                    .map(priority)
                    .sum::<usize>(),
                line.chars().collect::<HashSet<_>>(),
            )
        })
        .fold((0, vec![]), |(total, mut intersections), (sum, chars)| {
            intersections.push(chars);
            (total + sum, intersections)
        });

    let badge_sum: usize = intersections.chunks(3).map(|item| {
        let intermediate: HashSet<char> = item[0].intersection(&item[1]).copied().collect();
        let intersection: Vec<char> = intermediate.intersection(&item[2]).copied().collect();
        assert_eq!(intersection.len(), 1);
        priority(intersection[0])
    }).sum();

    println!("Sum: {} {}", total, badge_sum);
}
