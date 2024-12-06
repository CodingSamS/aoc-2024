use anyhow::{bail, Context};
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Clone, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn rotate(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[derive(Clone)]
struct Guard {
    direction: Direction,
    position: (usize, usize),
    max_x: usize,
    max_y: usize,
}

impl Guard {
    fn get_forward_position(&self) -> Option<(usize, usize)> {
        match self.direction {
            Direction::Up => {
                if self.position.1 == 0 {
                    None
                } else {
                    Some((self.position.0, self.position.1 - 1))
                }
            }
            Direction::Down => {
                if self.position.1 == self.max_y {
                    None
                } else {
                    Some((self.position.0, self.position.1 + 1))
                }
            }
            Direction::Left => {
                if self.position.0 == 0 {
                    None
                } else {
                    Some((self.position.0 - 1, self.position.1))
                }
            }
            Direction::Right => {
                if self.position.0 == self.max_x {
                    None
                } else {
                    Some((self.position.0 + 1, self.position.1))
                }
            }
        }
    }

    fn turn(&mut self) {
        self.direction = self.direction.rotate();
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
enum FieldType {
    Free,
    Obstacle,
}

fn puzzle01<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut guard_map = HashMap::new();
    let mut guard = Guard {
        direction: Direction::Up,
        position: (0, 0),
        max_x: 0,
        max_y: 0,
    };

    let mut y = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        let mut x = 0;
        for char in line.chars() {
            match char {
                '#' => guard_map.insert((x, y), FieldType::Obstacle),
                '.' => guard_map.insert((x, y), FieldType::Free),
                '^' => {
                    guard.position = (x, y);
                    guard_map.insert((x, y), FieldType::Free)
                }
                _ => bail!("character not supported"),
            };
            x += 1;
        }
        guard.max_x = x - 1;
        y += 1;
    }
    guard.max_y = y - 1;

    let mut visited = HashSet::new();
    loop {
        visited.insert(guard.position);
        match guard.get_forward_position() {
            Some((x, y)) => match guard_map.get(&(x, y)) {
                Some(field_type) => match field_type {
                    FieldType::Free => guard.position = (x, y),
                    FieldType::Obstacle => guard.turn(),
                },
                None => break,
            },
            None => break,
        }
    }

    Ok(visited.len())
}

fn puzzle02<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut guard_map = HashMap::new();
    let mut guard = Guard {
        direction: Direction::Up,
        position: (0, 0),
        max_x: 0,
        max_y: 0,
    };

    let mut y = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        let mut x = 0;
        for char in line.chars() {
            match char {
                '#' => guard_map.insert((x, y), FieldType::Obstacle),
                '.' => guard_map.insert((x, y), FieldType::Free),
                '^' => {
                    guard.position = (x, y);
                    guard_map.insert((x, y), FieldType::Free)
                }
                _ => bail!("character not supported"),
            };
            x += 1;
        }
        guard.max_x = x - 1;
        y += 1;
    }
    guard.max_y = y - 1;

    let mut cycle_counter = 0;
    let mut processed_count = 0;
    let number_of_keys = guard_map.keys().len();
    for key in guard_map.keys() {
        println!("processed ({}/{})", processed_count, number_of_keys);
        if key != &guard.position && guard_map.get(key).unwrap() != &FieldType::Obstacle {
            let mut guard_clone = guard.clone();
            let mut guard_map_clone = guard_map.clone();
            guard_map_clone.insert(*key, FieldType::Obstacle);

            // cycle detection
            let mut visited: HashMap<(usize, usize), Vec<Direction>> = HashMap::new();
            let mut is_cycle = false;
            'inner_loop: loop {
                visited
                    .entry(guard_clone.position)
                    .or_insert(Vec::new())
                    .push(guard_clone.direction.clone());
                match guard_clone.get_forward_position() {
                    Some((x, y)) => match guard_map_clone.get(&(x, y)) {
                        Some(field_type) => match field_type {
                            FieldType::Free => guard_clone.position = (x, y),
                            FieldType::Obstacle => guard_clone.turn(),
                        },
                        None => break 'inner_loop,
                    },
                    None => break 'inner_loop,
                }
                if let Some(direction_vec) = visited.get(&guard_clone.position) {
                    if direction_vec.contains(&guard_clone.direction) {
                        is_cycle = true;
                        break 'inner_loop;
                    }
                }
            }
            if is_cycle {
                cycle_counter += 1
            };
        }
        processed_count += 1;
    }

    Ok(cycle_counter)
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 41)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 6)
    }
}
