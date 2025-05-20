mod agent;
mod society;

use society::Society;

fn main() {
    let mut society = Society::new(100, 0.1, true);

    for round in 1..=10 {
        println!("Round {}", round);
        society.simulate_movement(2.0); // e.g. max move per round
        society.simulate_collisions(1.0); // e.g. proximity threshold
        society.simulate_transactions(200);
        society.apply_taxation();
        society.print_summary();
    }
}
