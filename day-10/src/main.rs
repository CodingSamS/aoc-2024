use anyhow::{bail, Context};
use std::{
    collections::{HashMap, HashSet},
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

struct TopographicMap {
    map: HashMap<Point, u32>,
    visited: HashSet<Point>,
}

impl TopographicMap {
    fn new() -> Self {
        TopographicMap {
            map: HashMap::new(),
            visited: HashSet::new(),
        }
    }

    fn insert_point(&mut self, point: Point, value: u32) {
        self.map.insert(point, value);
    }

    fn get_path_valid_neighbours(&self, point: &Point, path: &Vec<Point>) -> Vec<Point> {
        point
            .get_neighbours()
            .into_iter()
            .filter(|next_point| !path.contains(next_point))
            .filter(|next_point| self.is_valid_step(point, next_point))
            .collect()
    }

    fn get_unvisited_valid_neighbours(&self, point: Point) -> Vec<Point> {
        point
            .get_neighbours()
            .into_iter()
            .filter(|next_point| !self.visited.contains(next_point))
            .filter(|next_point| self.is_valid_step(&point, next_point))
            .collect()
    }

    fn is_valid_step(&self, point: &Point, next_point: &Point) -> bool {
        let Some(point_val) = self.map.get(point) else {
            return false;
        };
        let Some(next_point_val) = self.map.get(next_point) else {
            return false;
        };
        *point_val + 1 == *next_point_val
    }

    fn is_end_point(&self, point: &Point) -> anyhow::Result<bool> {
        match self.map.get(point) {
            Some(val) => {
                if *val == 9 {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => bail!("Point does not exist"),
        }
    }

    // visits a point. Returns true when the point is an end point (9), false if not. Error if the point was visited already or if it does not exist (This should not happen).
    fn visit_point(&mut self, point: Point) -> anyhow::Result<bool> {
        match self.visited.contains(&point) {
            true => bail!("Point was visited already. There seems to be an error when getting unvisited neighbours"),
            false => {
                self.visited.insert(point);
                match self.map.get(&point) {
                    Some(val) => if *val == 9 {
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                    None => bail!("Point does not exist")
                }
            }
        }
    }

    fn clear_visited(&mut self) {
        self.visited = HashSet::new();
    }
}

fn puzzle01<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let mut map = TopographicMap::new();
    let mut starting_points = Vec::new();

    let mut y = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        let mut x = 0;
        for c in line.chars() {
            let value = c.to_digit(10).context("char is no valid digit")?;
            if value == 0 {
                starting_points.push(Point::new(x, y));
            }
            map.insert_point(Point::new(x, y), value);
            x += 1;
        }
        y += 1;
    }

    let mut total_score = 0;

    for starting_point in starting_points {
        let mut trailhead_score = 0;
        let mut point_vec = vec![starting_point];
        while !point_vec.is_empty() {
            let point = point_vec.pop().context("Vec is empty but should not be")?;
            if map.visit_point(point)? {
                // point is an endpoint
                trailhead_score += 1;
            }
            point_vec.append(&mut map.get_unvisited_valid_neighbours(point));
        }
        map.clear_visited();
        total_score += trailhead_score;
    }

    Ok(total_score)
}

fn puzzle02<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let mut map = TopographicMap::new();
    let mut starting_points = Vec::new();

    let mut y = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        let mut x = 0;
        for c in line.chars() {
            let value = c.to_digit(10).context("char is no valid digit")?;
            if value == 0 {
                starting_points.push(Point::new(x, y));
            }
            map.insert_point(Point::new(x, y), value);
            x += 1;
        }
        y += 1;
    }

    let mut total_score = 0;

    for starting_point in starting_points {
        let mut trailhead_score = 0;
        let mut path_vec = vec![vec![starting_point]];
        while !path_vec.is_empty() {
            let path = path_vec.pop().context("Vec is empty but should not be")?;
            let last_point = path.last().context("Path is empty but should not be")?;
            if map.is_end_point(last_point)? {
                trailhead_score += 1;
            } else {
                for point in map.get_path_valid_neighbours(last_point, &path) {
                    let mut new_path = path.clone();
                    new_path.push(point);
                    path_vec.push(new_path);
                }
            }
        }
        total_score += trailhead_score;
    }

    Ok(total_score)
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 36)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 81)
    }
}
