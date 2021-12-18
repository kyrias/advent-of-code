use std::io::BufRead;


#[derive(Debug, PartialEq)]
struct Lanternfish {
    timer: usize,
}

impl Lanternfish {
    fn new(timer: usize) -> Self {
        Lanternfish {
            timer
        }
    }

    fn tick(self: &mut Self) -> MaybeSpawn {
        if self.timer == 0 {
            self.timer = 6;
            MaybeSpawn::Spawn
        } else {
            self.timer -= 1;
            MaybeSpawn::DontSpawn
        }
    }
}


#[derive(Debug, PartialEq)]
enum MaybeSpawn {
    DontSpawn,
    Spawn,
}


/// Simulate fish population growth over X days
fn simulate(fish: &mut Vec<Lanternfish>, days: usize) {
    for _ in 0..days {
        let spawn_num: usize = fish
            .iter_mut()
            .map(|f| f.tick())
            .map(|maybe| {
                if maybe == MaybeSpawn::Spawn { 1 } else { 0 }
            })
            .sum();
        for _ in 0..spawn_num {
            fish.push(Lanternfish::new(8));
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut input_lines = stdin.lock().lines();

    let timers = input_lines
        .next()
        .ok_or("no timer input given")??;

    let mut fish = Vec::new();
    for number in timers.split(",") {
        fish.push(Lanternfish::new(number.parse::<usize>()?));
    }

    simulate(&mut fish, 80);
    println!("Part 1: Number of fish after 80 days: {}", fish.len());

    simulate(&mut fish, 256-80);
    println!("Part 2: Number of fish after 256 days: {}", fish.len());

    Ok(())
}
