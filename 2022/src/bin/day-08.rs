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
}
