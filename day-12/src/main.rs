use anyhow::Context;
use std::{
    char,
    collections::HashMap,
    fmt::Debug,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {}, y: {})", self.x, self.y)
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }

    fn get_neighbours(&self) -> Vec<Point> {
        let mut neighbour_vec = Vec::new();
        if let Some(x_sub) = self.x.checked_sub(1) {
            neighbour_vec.push(Point::new(x_sub, self.y));
        }
        if let Some(x_add) = self.x.checked_add(1) {
            neighbour_vec.push(Point::new(x_add, self.y));
        }
        if let Some(y_sub) = self.y.checked_sub(1) {
            neighbour_vec.push(Point::new(self.x, y_sub));
        }
        if let Some(y_add) = self.y.checked_add(1) {
            neighbour_vec.push(Point::new(self.x, y_add));
        }
        neighbour_vec
    }
}

fn puzzle01<P>(filename: P) -> anyhow::Result<u64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let mut garden_map = HashMap::new();

    let mut y = 0;
    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let mut x = 0;
        for c in line.chars() {
            garden_map.insert(Point { x, y }, c);
            x += 1;
        }
        y += 1;
    }

    let mut fence_cost = 0;

    loop {
        let mut area = 0;
        let mut perimeter = 0;
        let Some(start_point) = garden_map.keys().next().copied() else {
            break;
        };
        let symbol = garden_map.remove(&start_point).unwrap();
        area += 1;
        let mut neighbours = start_point.get_neighbours();
        loop {
            match neighbours.pop() {
                Some(neighbour) => {
                    if let Some(neighbour_symbol) = garden_map.get(&neighbour) {
                        if &symbol == neighbour_symbol {
                            // neighbour is in the same garden -> increase the area and search in its neighbourhood
                            neighbours.append(&mut neighbour.get_neighbours());
                            area += 1;
                            // remove neighbour from map making sure to only search it once
                            garden_map.remove(&neighbour);
                        } else {
                            // neighbour is in a different garden -> increase perimeter
                            perimeter += 1;
                        }
                    }
                }
                None => break,
            }
        }

        let mut neighbour_points = vec![point];
    }

    Ok(fence_cost)
}

fn puzzle02<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    Ok(1)
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 1930)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 31)
    }
}
