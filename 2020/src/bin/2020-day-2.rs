use std::io::BufRead;

use nom::IResult;

#[derive(Debug)]
struct PasswordSpecification<'a> {
    min: usize,
    max: usize,
    letter: char,
    password: &'a str,
}

fn parse_line(input: &str) -> IResult<&str, PasswordSpecification> {
    let (input, min) = nom::combinator::map_res(nom::character::complete::digit1, |out: &str| {
        usize::from_str_radix(&out, 10)
    })(input)?;

    let (input, _) = nom::bytes::complete::tag("-")(input)?;

    let (input, max) = nom::combinator::map_res(nom::character::complete::digit1, |out: &str| {
        usize::from_str_radix(&out, 10)
    })(input)?;

    let (input, _) = nom::bytes::complete::tag(" ")(input)?;

    let (input, letter) = nom::character::complete::anychar(input)?;

    let (input, _) = nom::bytes::complete::tag(": ")(input)?;

    let (input, password) = nom::character::complete::not_line_ending(input)?;

    Ok((
        input,
        PasswordSpecification {
            min,
            max,
            letter,
            password,
        },
    ))
}

fn password_is_valid_down_the_street(spec: &PasswordSpecification) -> bool {
    let matches = spec.password.chars().filter(|&c| c == spec.letter).count();
    matches >= spec.min && matches <= spec.max
}

fn password_is_valid_north_pole_toboggan(spec: &PasswordSpecification) -> bool {
    let min_matches = match spec.password.chars().nth(spec.min - 1) {
        Some(c) => c == spec.letter,
        None => false,
    };

    let max_matches = match spec.password.chars().nth(spec.max - 1) {
        Some(c) => c == spec.letter,
        None => false,
    };

    min_matches ^ max_matches
}

fn main() {
    let stdin = std::io::stdin();
    let lines: Vec<String> = stdin.lock().lines().filter_map(|l| l.ok()).collect();

    let specs: Vec<PasswordSpecification> = lines
        .iter()
        .map(|l| parse_line(l.as_ref()))
        .filter_map(|l| l.ok())
        .map(|(_, spec)| spec)
        .collect();

    let valid_down_the_street = specs
        .iter()
        .map(password_is_valid_down_the_street)
        .filter(|&is_valid| is_valid)
        .count();
    println!("Valid passwords down the street: {}", valid_down_the_street);

    let valid_north_pole_toboggan = specs
        .iter()
        .map(password_is_valid_north_pole_toboggan)
        .filter(|&is_valid| is_valid)
        .count();
    println!(
        "Valid passwords at North Pole Toboggan: {}",
        valid_north_pole_toboggan
    );
}
