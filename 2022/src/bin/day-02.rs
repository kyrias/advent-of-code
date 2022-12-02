#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("Oh no"),
        }
    }
}

impl Outcome {
    fn score(self) -> usize {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Lose => 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn beats(self, other: Move) -> Outcome {
        match (self, other) {
            (Move::Rock, Move::Scissors) => Outcome::Win,
            (Move::Scissors, Move::Paper) => Outcome::Win,
            (Move::Paper, Move::Rock) => Outcome::Win,
            (a, b) if a == b => Outcome::Draw,
            _ => Outcome::Lose,
        }
    }

    fn calculate_move(wanted_outcome: Outcome, opponents_move: Move) -> Self {
        match (wanted_outcome, opponents_move) {
            (Outcome::Win, Move::Rock) => Move::Paper,
            (Outcome::Win, Move::Paper) => Move::Scissors,
            (Outcome::Win, Move::Scissors) => Move::Rock,
            (Outcome::Draw, _) => opponents_move,
            (Outcome::Lose, Move::Rock) => Move::Scissors,
            (Outcome::Lose, Move::Paper) => Move::Rock,
            (Outcome::Lose, Move::Scissors) => Move::Paper,
        }
    }

    fn score(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl From<&str> for Move {
    fn from(value: &str) -> Self {
        match value {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => panic!("Oh no"),
        }
    }
}

fn main() {
    let (part1_score, part2_score) = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut iter = line.split_whitespace();
            let first = iter.next().unwrap();
            let second = iter.next().unwrap();

            (Move::from(first), Move::from(second), Outcome::from(second))
        })
        .map(|(opponents_move, my_move, wanted_outcome)| {
            let part1 = my_move.score() + my_move.beats(opponents_move).score();

            let part2_my_move = Move::calculate_move(wanted_outcome, opponents_move);
            let part2 = part2_my_move.score() + part2_my_move.beats(opponents_move).score();

            (part1, part2)
        })
        .fold((0, 0), |(p1, p2), (n1, n2)| (p1 + n1, p2 + n2));
    println!("Scores: {} {}", part1_score, part2_score);
}
