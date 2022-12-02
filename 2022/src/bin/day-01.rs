fn main() {
    let (_, mut totals) =
        std::io::stdin()
            .lines()
            .fold((0, Vec::new()), |(mut total, mut totals), line| {
                let line = line.unwrap();
                if line.trim().is_empty() {
                    totals.push(total);
                    total = 0;
                } else {
                    let count: usize = line.parse().unwrap();
                    total += count
                }
                (total, totals)
            });

    totals.sort_by_key(|v| std::cmp::Reverse(*v));

    println!("Most calories: {}", totals[0]);
    println!("Top three    : {}", totals[..3].iter().sum::<usize>());
}
