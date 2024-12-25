use std::time::Instant;
mod day5;

fn main() {
    let now = Instant::now();

    {
        day5::main();
    }

    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
}
