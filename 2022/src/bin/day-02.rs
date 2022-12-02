#[derive(Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Win,
    Draw,
    Lose,
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

    fn score(self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let m = match value {
            "A" => Move::Rock,
            "B" => Move::Paper,
            "C" => Move::Scissors,
            "X" => Move::Rock,
            "Y" => Move::Paper,
            "Z" => Move::Scissors,
            _ => return Err(()),
        };
        Ok(m)
    }
}

fn main() {
    let score: usize = std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut mapped = line.split_whitespace().map(Move::try_from);
            (
                mapped.next().unwrap().unwrap(),
                mapped.next().unwrap().unwrap(),
            )
        })
        .map(|(opponent, mine)| {
            let outcome = mine.beats(opponent);
            mine.score() + outcome.score()
        })
        .sum();
    println!("Score: {}", score);
}
