use std::io::BufRead;


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let input_lines = stdin.lock().lines();

    let mut heightmap = Vec::new();
    let mut heightmap_width = 0;
    for line in input_lines {
        let line = line?;
        heightmap_width = line.len();
        for height in line.chars() {
            heightmap.push(height.to_digit(10).ok_or("heightmap out of spec")?);
        }
    }

    let heightmap_height = heightmap.len() / heightmap_width;
    let mut total_low_point_risk_level = 0;

    for (idx, height) in heightmap.iter().copied().enumerate() {
        let x = idx / heightmap_width;
        let y = idx % heightmap_width;

        if x > 0 && height >= heightmap[idx - heightmap_width] {
            continue
        }

        if x < heightmap_height-1 && height >= heightmap[idx + heightmap_width] {
            continue
        }

        if y > 0 && height >= heightmap[idx - 1] {
            continue
        }

        if y < heightmap_width-1 && height >= heightmap[idx + 1] {
            continue
        }

        total_low_point_risk_level += height + 1;
    }

    println!("Part 1: {}", total_low_point_risk_level);

    Ok(())
}
