use std::{
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

    fn get_next_stones(&self) -> Vec<Stone> {
        if let Some(stone_vec) = self.try_get_zero_stone() {
            stone_vec
        } else if let Some(stone_vec) = self.try_get_stone_halves() {
            stone_vec
        } else {
            vec![Stone {
                number: self.number * 2024,
                blinks: self.blinks + 1,
            }]
        }
    }

    fn try_get_zero_stone(&self) -> Option<Vec<Stone>> {
        if self.number == 0 {
            Some(vec![Stone {
                number: 1,
                blinks: self.blinks + 1,
            }])
        } else {
            None
        }
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

fn calc_resulting_stones(stone: Stone, max_number_of_blinks: usize) -> u128 {
    let mut number_of_resulting_stones = 0;

    let mut stone_vec = vec![stone];
    let mut time_now = SystemTime::now();
    loop {
        match stone_vec.pop() {
            Some(stone) => {
                for next_stone in stone.get_next_stones() {
                    if stone.blinks < max_number_of_blinks {
                        stone_vec.push(next_stone);
                    } else {
                        number_of_resulting_stones += 1;
                        let time_delta = time_now.elapsed().unwrap().as_secs();
                        if 300 < time_delta {
                            println!(
                                "Number of stones processed: {}\nDuration: {}",
                                number_of_resulting_stones, time_delta
                            );
                            time_now = SystemTime::now();
                        }
                    }
                }
            }
            None => break, // all stones handled
        }
    }

    number_of_resulting_stones
}

fn puzzle02<P>(filename: P, number_of_blinks: usize) -> anyhow::Result<u128>
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

    let mut number_of_stones = 0;

    for stone in stone_line {
        let start_time = SystemTime::now();
        number_of_stones += calc_resulting_stones(stone, number_of_blinks);
        println!(
            "Duration for stone: {}s",
            start_time.elapsed().unwrap().as_secs()
        );
    }

    Ok(number_of_stones)
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
