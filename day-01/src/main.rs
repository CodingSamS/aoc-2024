use anyhow::Context;
use regex::Regex;
use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn puzzle01<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)\s*(\d+)")?;

    let mut left_lines: Vec<i64> = Vec::new();
    let mut right_lines: Vec<i64> = Vec::new();
    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let capture = line_regex.captures(&line).context("no line capture")?;
        left_lines.push(capture[1].parse()?);
        right_lines.push(capture[2].parse()?);
    }

    left_lines.sort_unstable();
    right_lines.sort_unstable();

    Ok(left_lines
        .iter()
        .zip(right_lines.iter())
        .map(|(left, right)| (right - left).abs())
        .sum())
}

fn puzzle02<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)\s*(\d+)")?;

    let mut left_lines: Vec<i64> = Vec::new();
    let mut right_lines: Vec<i64> = Vec::new();
    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let capture = line_regex.captures(&line).context("no line capture")?;
        left_lines.push(capture[1].parse()?);
        right_lines.push(capture[2].parse()?);
    }

    Ok(left_lines
        .iter()
        .map(|value| {
            value * i64::try_from(right_lines.iter().filter(|v| v == &value).count()).unwrap()
        })
        .sum())
}

fn main() {
    println!("Solution 1: {}", puzzle01("data/data_1").unwrap());
    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 11)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 31)
    }
}
