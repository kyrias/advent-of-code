use std::io::BufRead;


struct School {
    /// Store population count for timers between 0 and 6.
    // The slot at index X represents a count of all fish with a timer value of X days.
    population: [usize; 7],

    /// Younglings that are still on their first two extra ticks.
    // The slot at index X represents a count of all fish with a timer value of X+6 days.
    younglings: [usize; 2],
}

impl School {
    fn new() -> Self {
        School {
            population: [0; 7],
            younglings: [0; 2],
        }
    }

    /// # Panics
    ///
    /// Panics if timer isn't 0 <= `timer` <= 8.
    fn spawn_fish(self: &mut Self, timer: u8) {
        if timer < 7 {
            self.population[timer as usize] += 1;
        } else if timer < 9 {
            self.younglings[timer as usize-7] += 1;
        } else {
            panic!("Tried to spawn a fish with a timer of {}, which is larger than the maximum allowed of 8.", timer);
        }
    }

    fn total_fish(self: &Self) -> usize {
        self.population.iter().sum::<usize>() + self.younglings.iter().sum::<usize>()
    }

    fn tick(self: &mut Self) {
        let new_young_fish = self.population[0];

        // Shift the whole of [pop0, pop1, ..., pop6, young0, young1] left one step to simulate
        // all fish getting a day older.
        self.population.rotate_left(1);
        self.population[6] += self.younglings[0];
        self.younglings.rotate_left(1);

        // The only fish with a timer of 8 are the ones that were supposed to be spawned today.
        self.younglings[1] = new_young_fish;
    }

}


/// Simulate the fish population growth over X days.
fn simulate_days(school: &mut School, days: usize) {
    for _ in 0..days {
        school.tick();
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let stdin = std::io::stdin();
    let mut input_lines = stdin.lock().lines();

    let timers = input_lines
        .next()
        .ok_or("no timer input given")??;

    let mut school = School::new();
    for number in timers.split(",") {
        school.spawn_fish(number.parse::<u8>()?);
    }

    simulate_days(&mut school, 80);
    println!("Stage 1: Number of fish after 80 days: {}", school.total_fish());

    simulate_days(&mut school, 256-80);
    println!("Stage 2: Number of fish after 256 days: {}", school.total_fish());

    Ok(())
}
