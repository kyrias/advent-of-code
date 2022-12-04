use std::ops::RangeInclusive;

fn to_range(assignments: &str) -> RangeInclusive<isize> {
    let mut iter = assignments.split('-');

    let start = iter.next().unwrap().parse().unwrap();
    let end = iter.next().unwrap().parse().unwrap();

    RangeInclusive::new(start, end)
}

fn main() {
    let ranges: Vec<_> = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut ranges: Vec<_> = line.split(',').map(to_range).collect();
            ranges.sort_by(|a, b| {
                let start = a.start().cmp(b.start());
                let end = b.end().cmp(a.end());
                start.then(end)
            });

            (ranges[0].clone(), ranges[1].clone())
        })
        .collect();

    let fully_contained = ranges
        .iter()
        .filter(|(min, max)| min.start() <= max.start() && min.end() >= max.end())
        .count();
    println!("Fully contained: {}", fully_contained);

    let any_overlap = ranges
        .iter()
        .filter(|(min, max)| min.contains(max.start()))
        .count();
    println!("Any overlap: {}", any_overlap);
}
