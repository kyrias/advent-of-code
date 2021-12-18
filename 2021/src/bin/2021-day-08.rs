use std::collections::HashSet;
use std::io::BufRead;


fn get_pattern_by_pred<P>(
    signal_patterns: &mut Vec<HashSet<char>>,
    predicate: P,
) -> Option<HashSet<char>> where P: Fn(&HashSet<char>) -> bool {
    let mut i = 0;
    while i < signal_patterns.len() {
        if predicate(&signal_patterns[i]) {
            let found = signal_patterns.remove(i);
            return Some(found);
        } else {
            i += 1;
        }
    }

    None
}


fn resolve_output_digits(line: &str) -> Result<Vec<usize>, Box<dyn std::error::Error>> {
    let mut line_parts = line.split(" | ");

    // First we have to figure out which patterns match to which digits.
    let mut signal_patterns: Vec<HashSet<char>> = line_parts
        .next()
        .ok_or("No signal patterns found")?
        .split(" ")
        .map(|pat| pat.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    let one = get_pattern_by_pred(&mut signal_patterns, |pat| pat.len() == 2).expect("to find pattern matching one");
    let four = get_pattern_by_pred(&mut signal_patterns, |pat| pat.len() == 4).expect("to find pattern matching four");
    let seven = get_pattern_by_pred(&mut signal_patterns, |pat| pat.len() == 3).expect("to find pattern matching seven");
    let eight = get_pattern_by_pred(&mut signal_patterns, |pat| pat.len() == 7).expect("to find pattern matching eight");

    let nine = get_pattern_by_pred(
        &mut signal_patterns,
        |pat| { pat.len() == 6 && (pat & &four) == four },
    ).expect("to find pattern matching nine");
    let zero = get_pattern_by_pred(
        &mut signal_patterns,
        |pat| { pat.len() == 6 && (pat & &one).len() == 2 },
    ).expect("to find pattern matching zero");
    let six = get_pattern_by_pred(&mut signal_patterns, |pat| pat.len() == 6).expect("to find pattern matching six");

    let three = get_pattern_by_pred(
        &mut signal_patterns,
        |pat| { pat.len() == 5 && (pat - &one).len() == 3 },
    ).expect("to find pattern matching three");
    let five = get_pattern_by_pred(
        &mut signal_patterns,
        |pat| { pat.len() == 5 && &(pat & &six) == pat },
    ).expect("to find pattern matching five");
    let two = get_pattern_by_pred(&mut signal_patterns, |pat| pat.len() == 5).expect("to find pattern matching two");

    // We should have removed all of the patterns from the list if we managed to find all digits.
    assert_eq!(signal_patterns.len(), 0);
    let digits = vec![zero, one, two, three, four, five, six, seven, eight, nine];


    // Then we map the output digits to actual digits according to the mapping we built above.
    let output_digits = line_parts
        .next()
        .ok_or("No output digits found")?
        .split(" ")
        .map(|pat| pat.chars().collect::<HashSet<char>>())
        .collect::<Vec<HashSet<char>>>();

    let mut output = Vec::new();
    for output_digit in output_digits.iter() {
        let digit = digits
            .iter()
            .enumerate()
            .find(|(_, segments)| segments == &output_digit)
            .map(|(idx, _)| idx)
            .ok_or("Unknown segment pattern in output digit")?;
        output.push(digit);
    }

    Ok(output)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let input_lines = stdin.lock().lines();

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input_lines {
        let mut number = 0;
        for digit in resolve_output_digits(&line?)? {
            if digit == 1 || digit == 4 || digit == 7 || digit == 8 {
                part1 += 1;
            }

            number = (number * 10) + digit;
        }

        part2 += number;
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}
