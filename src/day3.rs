use regex::Regex;
use std::fs;
use std::io;
use std::str::FromStr;

fn parse_file() -> Result<String, io::Error> {
    let content = match fs::read_to_string("./d3.txt") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    return Ok(content);
}

fn sum(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut sum = 0;

    for m in re.captures_iter(&input) {
        let x = m.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let y = m.get(2).unwrap().as_str().parse::<i32>().unwrap();

        sum += x * y;
    }

    return sum;
}

fn conditional_sum(input: &str) -> i32 {
    let re = Regex::new(r"(^|do\(\))(.*?)(don\'t\(\)|$)").unwrap();
    let mut total = 0;

    for part in re.captures_iter(input) {
        let instructions = part.get(2).unwrap().as_str();
        println!("{:?}\n", instructions);
        total += sum(instructions);
    }

    return total;
}

fn conditional_sum2(input: &str) -> i32 {
    let re = Regex::new(r"do\(\)|don\'t\(\)|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut total = 0;
    let mut enabled = true;

    for part in re.captures_iter(input) {
        enabled = match part.get(0).unwrap().as_str() {
            "do()" => true,
            "don't()" => false,
            _ => {
                if enabled {
                    let x = part.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let y = part.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    total += x * y;
                }

                enabled
            }
        };
    }

    return total;
}

pub fn main() {
    let input = parse_file().unwrap();
    let sum = sum(&input);
    let c_sum = conditional_sum2(&input.trim());
    println!("Sum: {:?}", sum);
    println!("Conditional sum: {:?}", c_sum);
}
