use std::io::BufRead;


#[derive(Clone, Copy, Debug)]
enum Cell {
    Unmarked(u8),
    Marked(u8),
}

impl Cell {
    fn is_marked(self: &Self) -> bool {
        match self {
            Cell::Unmarked(_) => false,
            Cell::Marked(_) => true,
        }
    }

    fn is_unmarked(self: &Self) -> bool {
        match self {
            Cell::Unmarked(_) => true,
            Cell::Marked(_) => false,
        }
    }

    fn get_value(self: &Self) -> u8 {
        match self {
            Cell::Unmarked(val) => *val,
            Cell::Marked(val) => *val,
        }
    }
}

impl Default for Cell {
    fn default() -> Self {
        Cell::Unmarked(0)
    }
}


#[derive(Debug, Default)]
struct Board {
    number: usize,
    board: [Cell; 5*5],
}

impl Board {
    fn parse_board(number: usize, lines: &mut impl Iterator<Item=Result<String, std::io::Error>>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut board = Self {
            number: number,
            ..Default::default()
        };

        for y in 0..5 {
            let board_line = lines.next().expect("to get a board line")?;
            for (x, digit) in board_line.split_ascii_whitespace().enumerate() {
                board.board[y*5 + x] = Cell::Unmarked(digit.parse::<u8>()?);
            }
        }

        Ok(board)
    }


    fn mark_number(self: &mut Self, number: u8) {
        for y in 0..5 {
            for x in 0..5 {
                match self.board[y*5 + x] {
                    Cell::Unmarked(val) if val == number => {
                        self.board[y*5 + x] = Cell::Marked(val);
                    },
                    _ => {},
                }
            }
        }
    }


    fn has_won(self: &Self) -> bool {
        // First check rows
        for y in 0..5 {
            let row = y*5;
            let marked = self.board[row .. row+5].iter().all(Cell::is_marked);
            if marked {
                return true;
            }
        }

        // Then check columns
        for x in 0..5 {
            let column = [
                self.board[0*5 + x],
                self.board[1*5 + x],
                self.board[2*5 + x],
                self.board[3*5 + x],
                self.board[4*5 + x],
            ];
            let marked = column.iter().all(Cell::is_marked);
            if marked {
                return true;
            }
        }

        false
    }


    fn get_score(self: &Self) -> usize {
        self.board
            .iter()
            .filter(|cell| cell.is_unmarked())
            .map(|cell| cell.get_value() as usize)
            .sum()
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut numbers: Vec<u8> = Vec::new();
    for number in lines.next().ok_or("did not read initial line of numbers")??.split(',') {
        numbers.push(number.parse()?);
    }

    let mut boards: Vec<Board> = Vec::new();
    loop {
        let maybe_empty_line = lines.next();
        if maybe_empty_line.is_none() {
            break;
        }
        assert_eq!(maybe_empty_line.unwrap().expect("to get empty line"), "");

        boards.push(Board::parse_board(boards.len(), &mut lines)?);
    }

    let mut winning_boards = Vec::new();
    for number in numbers {
        boards.iter_mut().map(|board| board.mark_number(number)).count();

        let mut i = 0;
        while i < boards.len() {
            if boards[i].has_won() {
                let board = boards.remove(i);
                let score = board.get_score() * number as usize;
                winning_boards.push((board.number, score));
            } else {
                i += 1
            }
        }
    }


    let (first_number, first_score) = winning_boards.first().expect("no first winning board found");
    println!("Part 1: First winner is board {} with {}", first_number, first_score);

    let (last_number, last_score) = winning_boards.last().expect("no last winning board found");
    println!("Part 2: First winner is board {} with {}", last_number, last_score);

    Ok(())
}
