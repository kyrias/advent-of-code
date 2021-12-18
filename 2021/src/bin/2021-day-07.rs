use std::io::BufRead;


enum FuelCalculationMethod {
    /// The sum off the absolute difference between every number and the target.
    AbsoluteDifference,

    /// The `N`th triangle number where `N` is the difference given by
    /// [`AbsoluteDifference`](FuelCalculationMethod::AbsoluteDifference).
    TriangleNumberDifference,
}


/// Calculate the fuel usage to align at target position according to the given
/// [`FuelCalculationMethod`].
fn calculate_fuel_usage(positions: &[isize], target_pos: isize, method: FuelCalculationMethod) -> isize {
    let mapped = positions
        .iter()
        .map(|pos| (*pos - target_pos).abs());

    match method {
        FuelCalculationMethod::AbsoluteDifference => {
            mapped.sum()
        },
        FuelCalculationMethod::TriangleNumberDifference => {
            mapped
                .map(|diff| (diff * (diff + 1)) / 2)
                .sum()
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let input = stdin
        .lock()
        .lines()
        .next()
        .ok_or("no timer input given")??;

    let mut count = 0;
    let mut sum = 0;
    let mut positions = Vec::new();
    for number in input.split(",") {
        let parsed = number.parse::<isize>()?;
        count += 1;
        sum += parsed;
        positions.push(parsed);
    }
    positions.sort();

    let median_low_pos = positions.len() / 2;
    let median_low = positions[median_low_pos];
    let fuel_usage_abs_diff = calculate_fuel_usage(
        &positions[..],
        median_low,
        FuelCalculationMethod::AbsoluteDifference,
    );
    println!("Part 1: fuel used: {}", fuel_usage_abs_diff);

    let mean = sum / count;
    let fuel_usage_trinagle = calculate_fuel_usage(
        &positions[..],
        mean,
        FuelCalculationMethod::TriangleNumberDifference,
    );
    println!("Part 2: fuel used: {}", fuel_usage_trinagle);

    Ok(())
}
