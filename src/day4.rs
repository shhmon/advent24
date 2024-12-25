use std::fs;
use std::io;

fn parse_file() -> Result<String, io::Error> {
    let content = match fs::read_to_string("./d4.txt") {
        Ok(content) => content,
        Err(e) => return Err(e),
    };

    return Ok(content);
}

fn get_moves() -> Vec<(i32, i32)> {
    let values = [-1, 0, 1];

    let permutations: Vec<(i32, i32)> = values
        .iter()
        .flat_map(|&x| values.iter().map(move |&y| (x as i32, y as i32)))
        .filter(|&m| m != (0, 0))
        .collect();

    return permutations;
}

fn investigate(map: &Vec<Vec<char>>, i: usize, j: usize, m: &(i32, i32)) -> bool {
    let mut buffer = vec![];
    let target = ['X', 'M', 'A', 'S'];

    target.iter().enumerate().for_each(|(idx, _c)| {
        let (ni, nj) = (
            (i as i32) + (idx as i32) * m.0,
            (j as i32) + (idx as i32) * m.1,
        );

        let next = match map.get(ni as usize) {
            Some(row) => row.get(nj as usize),
            None => None,
        };

        if next.is_none() {
            return;
        }

        buffer.push(*next.unwrap());
    });

    return buffer[..] == target[..];
}

fn count_xmas(input: &str) -> usize {
    let mut count = 0;
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let matches = match *value {
                'X' => get_moves()
                    .iter()
                    .filter(|m| investigate(&map, i, j, m))
                    .count(),
                _ => 0,
            };

            count += matches;
        }
    }

    return count;
}

fn investigate_x(map: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let diagonals = [[(-1, -1), (1, 1)], [(-1, 1), (1, -1)]];

    for d in diagonals {
        let mut buffer = vec![];

        for m in d {
            let (ni, nj) = ((i as i32) + m.0, (j as i32) + m.1);

            let next = match map.get(ni as usize) {
                Some(row) => row.get(nj as usize),
                None => None,
            };

            if next.is_none() {
                return false;
            }

            buffer.push(*next.unwrap());
        }

        if buffer[..] != ['M', 'S'] && buffer[..] != ['S', 'M'] {
            return false;
        }
    }

    return true;
}

fn count_x_mas(input: &str) -> usize {
    let mut count = 0;
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for (i, row) in map.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let m = match *value {
                'A' => investigate_x(&map, i, j),
                _ => false,
            };

            if m {
                count += 1;
            }
        }
    }

    return count;
}

pub fn main() {
    let input = parse_file().unwrap();
    let sum = count_x_mas(&input);
    print!("{}", sum);
}
