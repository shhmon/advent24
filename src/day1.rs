use std::fs;
use std::io;

fn parse_file() -> Result<(Vec<i32>, Vec<i32>), io::Error> {
    let content = match fs::read_to_string("./d1.txt") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    let mut left = Vec::new();
    let mut right = Vec::new();

    for row in content.split("\n") {
        let numbers: Vec<i32> = row
            .split(" ")
            .into_iter()
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .map(|v| v.parse().unwrap())
            .collect();

        match numbers[..] {
            [a, b] => {
                left.push(a);
                right.push(b)
            }
            _ => (),
        };
    }

    return Ok((left, right));
}

fn distance() -> Result<i32, io::Error> {
    let (mut left, mut right) = parse_file().unwrap();

    left.sort();
    right.sort();

    let sum: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(left_n, right_n)| i32::abs(left_n - right_n))
        .sum();

    return Ok(sum);
}

fn similarity_score() -> Result<i32, io::Error> {
    let (left, right) = parse_file().unwrap();

    let scores = left
        .iter()
        .map(|lv| *lv * right.iter().filter(|rv| *lv == **rv).count() as i32);

    return Ok(scores.sum());
}

pub fn main() {
    println!("Distance: {}", distance().unwrap());
    println!("Similarity score: {}", similarity_score().unwrap());
}
