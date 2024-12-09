use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn add(val: u64, mut tail: Vec<u64>, result: u64) -> bool {
    match tail.pop() {
        Some(next_val) => {
            let new_val = val + next_val;
            add(new_val, tail.clone(), result) || mul(new_val, tail.clone(), result)
        }
        None => val == result,
    }
}

fn mul(val: u64, mut tail: Vec<u64>, result: u64) -> bool {
    match tail.pop() {
        Some(next_val) => {
            let new_val = val * next_val;
            add(new_val, tail.clone(), result) || mul(new_val, tail.clone(), result)
        }
        None => val == result,
    }
}

fn add2(val: u64, mut tail: Vec<u64>, result: u64) -> bool {
    match tail.pop() {
        Some(next_val) => {
            let new_val = val + next_val;
            add2(new_val, tail.clone(), result)
                || mul2(new_val, tail.clone(), result)
                || concat2(new_val, tail, result)
        }
        None => val == result,
    }
}

fn mul2(val: u64, mut tail: Vec<u64>, result: u64) -> bool {
    match tail.pop() {
        Some(next_val) => {
            let new_val = val * next_val;
            add2(new_val, tail.clone(), result)
                || mul2(new_val, tail.clone(), result)
                || concat2(new_val, tail, result)
        }
        None => val == result,
    }
}

fn concat2(val: u64, mut tail: Vec<u64>, result: u64) -> bool {
    match tail.pop() {
        Some(next_val) => {
            let new_val = val * u64::pow(10, next_val.checked_ilog10().unwrap_or(0) + 1) + next_val;
            add2(new_val, tail.clone(), result)
                || mul2(new_val, tail.clone(), result)
                || concat2(new_val, tail, result)
        }
        None => val == result,
    }
}

fn puzzle01<P>(filename: P) -> anyhow::Result<u64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let line_regex = Regex::new(r"(\d+):(( \d+)+)")?;
    let mut results: Vec<u64> = Vec::new();
    let mut operands: Vec<Vec<u64>> = Vec::new();
    for line in buf.lines().filter_map(|x| x.ok()) {
        for captures in line_regex.captures_iter(&line) {
            results.push(captures[1].parse()?);
            let mut temp_vec: Vec<u64> = Vec::new();
            for element in captures[2].trim_start().split(" ") {
                temp_vec.push(element.parse()?);
            }
            temp_vec.reverse();
            operands.push(temp_vec);
        }
    }

    let mut calibration_result = 0;
    for i in 0..results.len() {
        if add(0, operands[i].clone(), results[i]) || mul(0, operands[i].clone(), results[i]) {
            calibration_result += results[i];
        }
    }

    Ok(calibration_result)
}

fn puzzle02<P>(filename: P) -> anyhow::Result<u64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let line_regex = Regex::new(r"(\d+):(( \d+)+)")?;
    let mut results: Vec<u64> = Vec::new();
    let mut operands: Vec<Vec<u64>> = Vec::new();
    for line in buf.lines().filter_map(|x| x.ok()) {
        for captures in line_regex.captures_iter(&line) {
            results.push(captures[1].parse()?);
            let mut temp_vec: Vec<u64> = Vec::new();
            for element in captures[2].trim_start().split(" ") {
                temp_vec.push(element.parse()?);
            }
            temp_vec.reverse();
            operands.push(temp_vec);
        }
    }

    let mut calibration_result = 0;
    for i in 0..results.len() {
        if add2(0, operands[i].clone(), results[i])
            || mul2(0, operands[i].clone(), results[i])
            || concat2(0, operands[i].clone(), results[i])
        {
            calibration_result += results[i];
        }
    }

    Ok(calibration_result)
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 3749)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 11387)
    }
}
