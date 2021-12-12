use std::io::BufRead;

use itertools::Either;


struct Line {
    x1: usize,
    y1: usize,
    x2: usize,
    y2: usize,
}


fn parse_usize<'a>(s: &'a str) -> Result<(usize, &'a str), Box<dyn std::error::Error>> {
    let delimiter = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    let number: usize = s[..delimiter].parse()?;
    Ok((number, &s[delimiter..]))
}


fn range(from: usize, to: usize) -> Either<Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>>,
                                           impl Iterator<Item = usize>> {
    if from < to {
        Either::Left(Either::Left(from ..= to))
    } else if from > to {
        Either::Left(Either::Right((to ..= from).rev()))
    } else {
        Either::Right(std::iter::repeat(from))
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let input_lines = stdin.lock().lines();

    let mut max_x = 0;
    let mut max_y = 0;
    let mut lines = Vec::new();
    for input_line in input_lines {
        let input_line = input_line?;
        let (x1, rest) = parse_usize(&input_line)?;
        let (y1, rest) = parse_usize(&rest[1..])?;

        let (x2, rest) = parse_usize(&rest[4..])?;
        let (y2, _rest) = parse_usize(&rest[1..])?;

        max_x = std::cmp::max(max_x, std::cmp::max(x1, x2)+1);
        max_y = std::cmp::max(max_y, std::cmp::max(y1, y2)+1);
        lines.push(Line { x1, y1, x2, y2 });
    }


    let mut vents: Vec<usize> = Vec::with_capacity(max_x * max_y);
    let row = vec![0; max_x];
    for _ in 0..max_y {
        vents.extend(&row);
    }

    for line in lines.iter() {
        // In part 1 we ignore diagonal lines.
        let diagonal = !(line.x1 == line.x2 || line.y1 == line.y2);
        if diagonal {
            continue
        }

        let xs = range(line.x1, line.x2);
        let ys = range(line.y1, line.y2);

        for (x, y) in xs.zip(ys) {
            vents[x + y*max_x] += 1;
        }
     }

    let overlapping = vents.iter().enumerate().filter(|(_, number)| **number > 1).map(|(idx, _)| idx).count();
    println!("Part 1: {}", overlapping);


    for line in lines.iter() {
        // In part 2 we only process diagonal lines.
        let diagonal = !(line.x1 == line.x2 || line.y1 == line.y2);
        if !diagonal {
            continue
        }

        let xs = range(line.x1, line.x2);
        let ys = range(line.y1, line.y2);

        for (x, y) in xs.zip(ys) {
            vents[x + y*max_x] += 1;
        }
     }

    let overlapping = vents.iter().enumerate().filter(|(_, number)| **number > 1).map(|(idx, _)| idx).count();
    println!("Part 2: {}", overlapping);

    Ok(())
}
