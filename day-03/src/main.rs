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
    let mut buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut line = String::new();
    buf.read_line(&mut line)?;

    Ok(line_regex
        .captures_iter(&line)
        .map(|captures| captures[1].parse::<i64>().unwrap() * captures[2].parse::<i64>().unwrap())
        .sum())
}

fn puzzle02<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut buf = io::BufReader::new(file);

    let replace_regex_1 = Regex::new(r"don't\(\).*?do\(\)")?;
    let replace_regex_2 = Regex::new(r"don't\(\).*")?;
    let line_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?;

    let mut line = String::new();
    buf.read_line(&mut line)?;

    let replaced_line = replace_regex_1.replace_all(&line, "");
    let replaced_line = replace_regex_2.replace_all(&replaced_line, "");

    Ok(line_regex
        .captures_iter(&replaced_line)
        .map(|captures| captures[1].parse::<i64>().unwrap() * captures[2].parse::<i64>().unwrap())
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
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 161)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_2").unwrap(), 48)
    }
}
