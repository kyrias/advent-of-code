use itertools::Itertools;
use odds::string::StrChunksWindows;

fn main() {
    let line = std::io::stdin().lines().next().unwrap().unwrap();
    let (start_of_packet_idx, _) = line
        .char_windows(4)
        .enumerate()
        .find(|(_, slice)| slice.chars().unique().count() == 4)
        .unwrap();
    println!("Start-of-packet at: {}", start_of_packet_idx + 4);

    let (start_of_message_idx, _) = line
        .char_windows(14)
        .enumerate()
        .find(|(_, slice)| slice.chars().unique().count() == 14)
        .unwrap();
    println!("Start-of-message at {}", start_of_message_idx + 14);
}
