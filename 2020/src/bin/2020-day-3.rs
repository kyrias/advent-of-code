use std::io::BufRead;

#[derive(PartialEq)]
enum Tile {
    Open,
    Tree,
}

fn find_trees_for_slope(right: u8, down: u8, map: &[Vec<Tile>]) -> u64 {
    let (_, count) =
        map.iter()
            .step_by(down.into())
            .fold((0, 0), |(x, count), line: &Vec<Tile>| {
                (
                    x + right as usize,
                    if line[x as usize % line.len()] == Tile::Tree {
                        count + 1
                    } else {
                        count
                    },
                )
            });

    count
}

fn main() {
    let stdin = std::io::stdin();
    let map: Vec<Vec<Tile>> = stdin
        .lock()
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .map(|c| if c == '#' { Tile::Tree } else { Tile::Open })
                .collect()
        })
        .collect();

    let right_3_down_1 = find_trees_for_slope(3, 1, &map);
    println!("Number of trees encountered: {}", right_3_down_1);

    let product: u64 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .iter()
        .map(|(x, y)| find_trees_for_slope(*x, *y, &map))
        .product();
    println!("Product of multiple slopes: {}", product);
}
