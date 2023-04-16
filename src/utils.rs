use rand::RngCore;

// *** Was in main.c ***

// Pick a random number.
pub fn rnd(range: usize) -> usize {
    let mut rng = rand::thread_rng();
    return if range == 0 { 0 } else { rng.next_u64() as usize % range };
}

// Roll a number of dice
pub fn roll(number: usize, sides: usize) -> usize {
    let mut total = 0;
    for _ in 0..number {
        total += rnd(sides) + 1;
    }
    return total;
}