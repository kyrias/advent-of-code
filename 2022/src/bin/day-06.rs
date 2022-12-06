use itertools::Itertools;
use odds::string::StrChunksWindows;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let (idx, _) = line
        .char_windows(4)
        .enumerate()
        .find(|(_, slice)| slice.chars().unique().count() == 4)
        .unwrap();
    println!("First no-duplicate sequence at index {}", idx + 4);
}
