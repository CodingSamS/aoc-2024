use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};
use thiserror::Error;

#[derive(Error, Debug)]
enum XmasError {
    #[error("Could not parse char to XMAS struct")]
    XmasParseFailed,
}

enum XMAS {
    X,
    M,
    A,
    S,
}

enum NextValue {
    IsValid,
    IsXmas,
    IsNotValid,
}

impl XMAS {
    fn check_next(&self, next: &XMAS) -> NextValue {
        match self {
            XMAS::X => match next {
                XMAS::M => (NextValue::IsValid),
                _ => Ok(NextValue::IsNotValid),
            },
            XMAS::M => match next {
                XMAS::A => Ok(NextValue::IsValid),
                _ => Ok(NextValue::IsNotValid),
            },
            XMAS::A => match next {
                XMAS::S => Ok(NextValue::IsXmas),
                _ => Ok(NextValue::IsNotValid),
            },
            XMAS::S => Ok(NextValue::IsNotValid),
        }
    }
}

impl TryFrom<char> for XMAS {
    type Error = XmasError;
    fn try_from(value: char) -> Result<XMAS, XmasError> {
        match value {
            'X' => Ok(XMAS::X),
            'M' => Ok(XMAS::M),
            'A' => Ok(XMAS::A),
            'S' => Ok(XMAS::S),
            _ => Err(XmasError::XmasParseFailed),
        }
    }
}

fn puzzle01<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut xmas_vec: Vec<Vec<XMAS>> = Vec::new();
    for line in buf.lines().filter_map(|x| x.ok()) {
        let xmas_inner_vec: Result<Vec<XMAS>, _> =
            line.chars().map(|c| XMAS::try_from(c)).collect();
        xmas_vec.push(xmas_inner_vec?);
    }

    let mut counter = 0;

    // check horizontally
    for x in 0..xmas_vec.len() {
        let mut start = &xmas_vec[x][0];
        let mut count = 0;
        for y in 1..xmas_vec[x].len() {
            let next = &xmas_vec[x][y];
            match start.check_next(next) {}
        }
    }

    Ok(1)
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
    //    println!("Solution 2: {}", puzzle02("data/data_1").unwrap());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 18)
    }
    /*
    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 4)
    }*/
}
