use std::time::Instant;
mod day6;
mod utils;

fn main() {
    let now = Instant::now();

    {
        day6::main();
    }

    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
}
