use anyhow::Context;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
    slice::Iter,
};

const MAX_STEP_SIZE: i64 = 3;

fn check_ascending(number: i64, mut number_iter: Iter<i64>, problem_dampener: bool) -> bool {
    if let Some(next_number) = number_iter.next() {
        let diff = next_number - number;
        if 0 < diff && diff <= MAX_STEP_SIZE {
            check_ascending(*next_number, number_iter, problem_dampener)
        } else if problem_dampener {
            check_ascending(number, number_iter.clone(), false)
        } else {
            false
        }
    } else {
        true
    }
}

fn check_descending(number: i64, mut number_iter: Iter<i64>, problem_dampener: bool) -> bool {
    if let Some(next_number) = number_iter.next() {
        let diff = number - next_number;
        if 0 < diff && diff <= MAX_STEP_SIZE {
            check_descending(*next_number, number_iter, problem_dampener)
        } else if problem_dampener {
            check_descending(number, number_iter.clone(), false)
        } else {
            false
        }
    } else {
        true
    }
}

fn puzzle01<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut number_of_safe_reports = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        let numbers_result: Result<Vec<_>, _> =
            line.split_whitespace().map(|n| n.parse::<i64>()).collect();
        let numbers = numbers_result?;
        let mut number_iter = numbers.iter();
        let first_number: i64 = *number_iter.next().context("no first value")?;
        if let Some(second_number) = number_iter.next() {
            let diff = second_number - first_number;
            if diff < 0 && -MAX_STEP_SIZE <= diff {
                if check_descending(*second_number, number_iter, false) {
                    number_of_safe_reports += 1;
                }
            } else if 0 < diff && diff <= MAX_STEP_SIZE {
                if check_ascending(*second_number, number_iter, false) {
                    number_of_safe_reports += 1;
                }
            }
        } else {
            number_of_safe_reports += 1;
        }
    }

    Ok(number_of_safe_reports)
}

fn puzzle02<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut number_of_safe_reports = 0;
    for line in buf.lines().filter_map(|x| x.ok()) {
        let numbers_result: Result<Vec<_>, _> =
            line.split_whitespace().map(|n| n.parse::<i64>()).collect();
        let numbers = numbers_result?;
        let mut number_iter = numbers.iter();
        let first_number = number_iter.next().context("no first value")?;
        if check_ascending(*first_number, number_iter.clone(), true)
            || check_descending(*first_number, number_iter.clone(), true)
        {
            number_of_safe_reports += 1;
        } else {
            let second_number = number_iter.next().context("no second value")?;
            if check_ascending(*second_number, number_iter.clone(), false)
                || check_descending(*second_number, number_iter.clone(), false)
            {
                number_of_safe_reports += 1;
            }
        }
    }

    Ok(number_of_safe_reports)
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 2)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 4)
    }
}
