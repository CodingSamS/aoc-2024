use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
    time::SystemTime,
};

#[derive(Debug)]
struct Stone {
    number: u64,
    blinks: usize,
}

impl Stone {
    fn new(number: u64) -> Self {
        Stone { number, blinks: 1 }
    }

    fn is_zero(&self) -> bool {
        self.number == 0
    }

    fn try_get_stone_halves(&self) -> Option<Vec<Stone>> {
        let number_of_digits = self.number.checked_ilog10().unwrap_or(0) + 1;
        if number_of_digits % 2 == 0 {
            let divisor = u64::pow(10, number_of_digits / 2);
            let first_half = self.number / divisor;
            let second_half = self.number % divisor;
            Some(vec![
                Stone {
                    number: first_half,
                    blinks: self.blinks + 1,
                },
                Stone {
                    number: second_half,
                    blinks: self.blinks + 1,
                },
            ])
        } else {
            None
        }
    }
}

fn puzzle01<P>(filename: P, number_of_blinks: usize) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let mut stone_line = Vec::new();

    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        for line_part in line.split(" ") {
            stone_line.push(Stone::new(line_part.trim().parse()?));
        }
    }

    for i in 1..=number_of_blinks {
        let start_time = SystemTime::now();
        stone_line = stone_line
            .into_iter()
            .map(|stone| {
                if stone.is_zero() {
                    vec![Stone::new(1)]
                } else if let Some(stone_halves) = stone.try_get_stone_halves() {
                    stone_halves
                } else {
                    vec![Stone::new(stone.number * 2024)]
                }
            })
            .flatten()
            .collect();
        let duration = start_time.elapsed().unwrap();
        println!(
            "Number of blinks: {}   Number of stones: {}   (Duration: {}s)",
            i,
            stone_line.len(),
            duration.as_secs()
        );
    }

    Ok(stone_line.len())
}

fn puzzle02<P>(filename: P, number_of_blinks: usize) -> anyhow::Result<u64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let mut stone_map: HashMap<u64, u64> = HashMap::new();

    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        for line_part in line.split(" ") {
            *stone_map.entry(line_part.trim().parse()?).or_insert(0) += 1;
        }
    }

    for i in 1..=number_of_blinks {
        let start_time = SystemTime::now();
        let mut new_stone_map = HashMap::new();
        for (stone_number, count) in stone_map {
            if stone_number == 0 {
                *new_stone_map.entry(stone_number + 1).or_insert(0) += count;
            } else {
                let number_of_digits = stone_number.checked_ilog10().unwrap_or(0) + 1;
                if number_of_digits % 2 == 0 {
                    let divisor = u64::pow(10, number_of_digits / 2);
                    let first_half = stone_number / divisor;
                    let second_half = stone_number % divisor;
                    *new_stone_map.entry(first_half).or_insert(0) += count;
                    *new_stone_map.entry(second_half).or_insert(0) += count;
                } else {
                    *new_stone_map.entry(stone_number * 2024).or_insert(0) += count;
                }
            }
        }
        stone_map = new_stone_map;
        println!(
            "Iteration: {} Stone Count: {} Duration: {}s",
            i,
            stone_map.values().sum::<u64>(),
            start_time.elapsed().unwrap().as_secs()
        );
    }

    Ok(stone_map.values().sum())
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1", 25).unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1", 75).unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1", 25).unwrap(), 55312)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1", 25).unwrap(), 55312)
    }
}
