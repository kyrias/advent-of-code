use itertools::Itertools;
use ndarray::{Array2, Axis};

fn is_visible(map: &Array2<u32>, row: usize, column: usize) -> bool {
    let height = map.row(row)[column];
    let (left, right) = map.row(row).split_at(Axis(0), column);
    let (top, bottom) = map.column(column).split_at(Axis(0), row);

    top.iter().all(|t| *t < height)
        || bottom.iter().skip(1).all(|t| *t < height)
        || left.iter().all(|t| *t < height)
        || right.iter().skip(1).all(|t| *t < height)
}

fn scenic_score(map: &Array2<u32>, row: usize, column: usize) -> usize {
    let height = map.row(row)[column];
    let (left, right) = map.row(row).split_at(Axis(0), column);
    let (top, bottom) = map.column(column).split_at(Axis(0), row);

    let viewing_distance = |(acc, cont): (usize, bool), t: &u32| -> (usize, bool) {
        if !cont {
            (acc, cont)
        } else if *t < height {
            (acc + 1, true)
        } else {
            (acc + 1, false)
        }
    };

    let (top_distance, _) = top.iter().rev().fold((0, true), viewing_distance);
    let (bottom_distance, _) = bottom.iter().skip(1).fold((0, true), viewing_distance);

    let (left_distance, _) = left.iter().rev().fold((0, true), viewing_distance);
    let (right_distance, _) = right.iter().skip(1).fold((0, true), viewing_distance);

    top_distance * left_distance * bottom_distance * right_distance
}

fn main() {
    let mut lines = std::io::stdin()
        .lines()
        .map(|line| line.unwrap())
        .peekable();
    let width = lines.peek().unwrap().len();
    let data = lines
        .flat_map(|line| line.chars().map(|c| c as u32 - '0' as u32).collect_vec())
        .collect_vec();
    let height = data.len() / width;
    let map = Array2::from_shape_vec((height, width), data).unwrap();

    let border_trees = (height + width) * 2 - 4;
    let visible_trees = (1..height - 1)
        .cartesian_product(1..width - 1)
        .filter(|(row, column)| is_visible(&map, *row, *column))
        .count();
    println!("Visible trees: {}", border_trees + visible_trees);

    let highest_scenic_score = (0..height)
        .cartesian_product(0..width)
        .map(|(row, column)| scenic_score(&map, row, column))
        .max()
        .unwrap();
    println!("Highest scenic score: {}", highest_scenic_score);
}
