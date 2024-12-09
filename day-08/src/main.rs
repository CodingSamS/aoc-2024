use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn puzzle01<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut antennas = HashMap::new();

    let mut x_len = 0;
    let mut y_len = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        x_len = 0;
        for char in line.chars() {
            match char {
                '.' => (),
                _ => antennas
                    .entry(char)
                    .or_insert(Vec::new())
                    .push((x_len, y_len)),
            };
            x_len += 1;
        }
        y_len += 1;
    }
    let x_range = 0..x_len;
    let y_range = 0..y_len;

    let mut antinodes = HashSet::new();
    for vec in antennas.values() {
        for point in vec {
            for other_point in vec {
                if point != other_point {
                    let antinode_point = (
                        point.0 + (point.0 - other_point.0),
                        point.1 + (point.1 - other_point.1),
                    );
                    if x_range.contains(&antinode_point.0) && y_range.contains(&antinode_point.1) {
                        antinodes.insert(antinode_point);
                    }
                }
            }
        }
    }

    Ok(antinodes.len())
}

fn puzzle02<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut antennas = HashMap::new();

    let mut x_len = 0;
    let mut y_len = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        x_len = 0;
        for char in line.chars() {
            match char {
                '.' => (),
                _ => antennas
                    .entry(char)
                    .or_insert(Vec::new())
                    .push((x_len, y_len)),
            };
            x_len += 1;
        }
        y_len += 1;
    }
    let x_range = 0..x_len;
    let y_range = 0..y_len;

    let mut antinodes = HashSet::new();
    for vec in antennas.values() {
        for point in vec {
            for other_point in vec {
                if point == other_point {
                    antinodes.insert(*point);
                } else {
                    let x_diff = point.0 - other_point.0;
                    let y_diff = point.1 - other_point.1;
                    let mut antinode_point = (point.0 + x_diff, point.1 + y_diff);
                    while x_range.contains(&antinode_point.0) && y_range.contains(&antinode_point.1)
                    {
                        antinodes.insert(antinode_point);
                        antinode_point.0 += x_diff;
                        antinode_point.1 += y_diff;
                    }
                }
            }
        }
    }

    Ok(antinodes.len())
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 14)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 34)
    }
}
