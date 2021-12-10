use std::cmp::Ordering;
use std::io::BufRead;


fn most_common_bit(numbers: &Vec<String>, bit: usize) -> Ordering {
    let mut zeros = 0;
    let mut ones = 0;

    for number in numbers {
        if number.chars().nth(bit).expect("should have at least this many bits") == '0' {
            zeros += 1
        } else {
            ones += 1
        }
    }

    zeros.cmp(&ones)
}


fn calculate_rates(most_common: &Vec<Ordering>) -> (usize, usize) {
    let mut gamma_rate = 0usize;
    let mut epsilon_rate = 0usize;
    for order in most_common {
        gamma_rate <<= 1;
        epsilon_rate <<= 1;
        match order {
            Ordering::Less => {
                gamma_rate |= 1;
            },
            _ => {
                epsilon_rate |= 1;
            },
        }
    }

    (gamma_rate, epsilon_rate)
}


fn calculate_stage2<BS: Fn(&Vec<String>, usize) -> char>(ratings: &Vec<String>, bit_selector: BS) -> Result<usize, Box<dyn std::error::Error>> {
    let mut filtered = ratings.clone();

    for i in 0..ratings[0].len() {
        let bit = bit_selector(&filtered, i);

        filtered.retain(|number| {
            number.chars().nth(i).expect("should always have this many bits") == bit
        });

        if filtered.len() <= 1 {
            break;
        }
    }

    let filtered_rating = usize::from_str_radix(&filtered[0], 2)?;

    Ok(filtered_rating)
}


fn oxygen_generator_rating_bit_criteria(ratings: &Vec<String>, bit: usize) -> char {
    match most_common_bit(&ratings, bit) {
        Ordering::Less => '1',
        Ordering::Equal => '1',
        Ordering::Greater => '0',
    }
}


fn co2_scrubber_rating_bit_criteria(ratings: &Vec<String>, bit: usize) -> char {
    match most_common_bit(&ratings, bit) {
        Ordering::Less => '0',
        Ordering::Equal => '0',
        Ordering::Greater => '1',
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut report = Vec::new();
    for line in std::io::stdin().lock().lines() {
        report.push(line?);
    }

    let width = report[0].len();
    let mut most_common = Vec::with_capacity(width);

    for i in 0..width {
        most_common.push(most_common_bit(&report, i));
    }


    let (gamma, epsilon) = calculate_rates(&most_common);
    println!("Part 1: {}", gamma * epsilon);


    let oxygen_generator_rating = calculate_stage2(&report, oxygen_generator_rating_bit_criteria)?;
    let co2_scrubber_rating = calculate_stage2(&report, co2_scrubber_rating_bit_criteria)?;

    println!("Part 2: {}", oxygen_generator_rating * co2_scrubber_rating);

    Ok(())
}
