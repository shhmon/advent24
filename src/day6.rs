use crate::utils::read_file;

#[derive(Debug)]
struct Map {
    data: Vec<char>,
    columns: usize,
    position: (usize, usize),
    rotation: (i32, i32),
}

impl Map {
    fn new(input: &str) -> Self {
        let columns = input.lines().next().unwrap().chars().count();
        let data = input
            .lines()
            .flat_map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<char>>();

        let p = data.iter().position(|v| *v == '^').unwrap();
        let position = (p / columns, p % columns);

        Self {
            data,
            columns,
            position,
            rotation: (-1, 0),
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&char> {
        self.data.get(row * self.columns + col)
    }

    fn rotate(&mut self) {
        self.rotation = match self.rotation {
            (-1, 0) => (0, 1),
            (0, 1) => (1, 0),
            (1, 0) => (0, -1),
            (0, -1) => (-1, 0),
            _ => panic!("Invalid rotation"),
        };
    }

    fn step(&mut self) -> Option<(usize, usize)> {
        let (nr, nc) = (
            (self.position.0 as i32) + self.rotation.0,
            (self.position.1 as i32) + self.rotation.1,
        );

        if nr < 0 || nc < 0 {
            return None;
        }

        match self.get(nr as usize, nc as usize) {
            Some('.') | Some('^') => {
                self.position = (nr as usize, nc as usize);
            }
            Some('#') => {
                self.rotate();
            }
            _ => {
                return None;
            }
        }

        return Some(self.position);
    }
}

pub fn main() {
    let mut map = Map::new(&read_file("./d6.txt").unwrap());
    let mut buffer = vec![map.position];

    loop {
        match map.step() {
            Some(p) => {
                if !buffer.contains(&p) {
                    buffer.push(p);
                }
            }
            None => {
                break;
            }
        }
    }

    println!("{}", buffer.len())
}
