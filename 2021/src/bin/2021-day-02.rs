use std::io::BufRead;


enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

impl Command {
    fn from(s: &str) -> Result<Command, Box<dyn std::error::Error>> {
        let mut iter = s.split_ascii_whitespace();
        let command = iter.next().ok_or("Missing command")?;
        let number = iter.next().ok_or("Missing number")?.parse::<usize>()?;

        match command {
            "forward" => Ok(Command::Forward(number)),
            "down" => Ok(Command::Down(number)),
            "up" => Ok(Command::Up(number)),
            _ => Err(format!("Invalid command {:?}", command))?,
        }
    }
}


fn parse_command(line: &String) -> Result<Command, Box<dyn std::error::Error>> {
    Command::from(line)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut lines: Vec<String> = Vec::new();
    for line in std::io::stdin().lock().lines() {
        lines.push(line?);
    }

    let mut horizontal = 0;
    let mut depth = 0;
    for command in lines.iter().map(parse_command) {
        match command? {
            Command::Forward(number) => {
                horizontal += number
            },
            Command::Down(number) => {
                depth += number
            },
            Command::Up(number) => {
                depth -= number
            },
        }
    }

    println!("Part 1: {}", horizontal * depth);


    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;
    for command in lines.iter().map(parse_command) {
        match command? {
            Command::Forward(number) => {
                horizontal += number;
                depth += aim * number;
            },
            Command::Down(number) => {
                aim += number
            },
            Command::Up(number) => {
                aim -= number
            },
        }
    }

    println!("Part 2: {}", horizontal * depth);

    Ok(())
}
