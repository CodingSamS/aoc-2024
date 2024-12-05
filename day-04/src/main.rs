use anyhow::Context;
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

#[derive(Debug, Clone)]
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

fn is_mas(
    center: &XMAS,
    upper_left: &XMAS,
    upper_right: &XMAS,
    lower_left: &XMAS,
    lower_right: &XMAS,
) -> bool {
    match center {
        XMAS::A => match (upper_left, lower_right) {
            (XMAS::M, XMAS::S) => match (upper_right, lower_left) {
                (XMAS::M, XMAS::S) => true,
                (XMAS::S, XMAS::M) => true,
                _ => false,
            },
            (XMAS::S, XMAS::M) => match (upper_right, lower_left) {
                (XMAS::M, XMAS::S) => true,
                (XMAS::S, XMAS::M) => true,
                _ => false,
            },
            _ => false,
        },
        _ => false,
    }
}

impl XMAS {
    fn check_next(&self, next: &XMAS) -> NextValue {
        match self {
            XMAS::X => match next {
                XMAS::M => NextValue::IsValid,
                _ => NextValue::IsNotValid,
            },
            XMAS::M => match next {
                XMAS::A => NextValue::IsValid,
                _ => NextValue::IsNotValid,
            },
            XMAS::A => match next {
                XMAS::S => NextValue::IsXmas,
                _ => NextValue::IsNotValid,
            },
            XMAS::S => NextValue::IsNotValid,
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

fn xmas_count<'a, T>(v_iter: &mut T) -> Result<i64, XmasError>
where
    T: Iterator<Item = &'a XMAS>,
{
    let Some(mut start) = v_iter.next() else {
        return Err(XmasError::XmasParseFailed);
    };
    let mut counter = 0;
    let mut count = 0;

    for next in v_iter {
        match start.check_next(next) {
            NextValue::IsValid => count += 1,
            NextValue::IsNotValid => count = 0,
            NextValue::IsXmas => {
                if count == 2 {
                    counter += 1;
                }
                count = 0
            }
        }
        start = next;
    }

    Ok(counter)
}

fn puzzle01<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut xmas_vec_horizontal: Vec<Vec<XMAS>> = Vec::new();
    for line in buf.lines().filter_map(|x| x.ok()) {
        let xmas_inner_vec: Result<Vec<XMAS>, _> =
            line.chars().map(|c| XMAS::try_from(c)).collect();
        xmas_vec_horizontal.push(xmas_inner_vec?);
    }

    let rows = xmas_vec_horizontal.len();
    let columns = xmas_vec_horizontal
        .get(0)
        .context("not enough columns")?
        .len();

    // create vertical vec
    let mut xmas_vec_vertical: Vec<Vec<XMAS>> = Vec::new();
    for _ in 0..columns {
        xmas_vec_vertical.push(Vec::new());
    }
    for x in 0..rows {
        for y in 0..columns {
            xmas_vec_vertical[y].push(xmas_vec_horizontal[x][y].clone());
        }
    }

    // create diagonal vec left to right
    let mut xmas_vec_diagonal_left_to_right: Vec<Vec<XMAS>> = Vec::new();
    let mut temp_row = rows - 1;
    let mut temp_col = 0;
    while temp_col < columns {
        let mut y = temp_col;
        let mut temp_vec = Vec::new();
        for x in temp_row..rows {
            if x < rows && y < columns {
                temp_vec.push(xmas_vec_horizontal[x][y].clone())
            } else {
                break;
            }
            y += 1;
        }
        xmas_vec_diagonal_left_to_right.push(temp_vec);
        if 0 < temp_row {
            // decrease rows -> moving up in cols first
            temp_row -= 1;
        } else {
            // increase cols -> now moving rigth to the last col
            temp_col += 1;
        }
    }

    // create diagonal vec right to left
    let mut xmas_vec_diagonal_right_to_left: Vec<Vec<XMAS>> = Vec::new();
    let mut temp_row = rows - 1;
    let mut temp_col = columns - 1;
    while 0 < temp_col {
        let mut y = temp_col;
        let mut temp_vec = Vec::new();
        for x in temp_row..rows {
            if x < rows {
                temp_vec.push(xmas_vec_horizontal[x][y].clone())
            } else {
                break;
            }
            if y == 0 {
                break;
            } else {
                y -= 1;
            }
        }
        xmas_vec_diagonal_right_to_left.push(temp_vec);
        if 0 < temp_row {
            temp_row -= 1;
        } else {
            temp_col -= 1;
        }
    }

    let mut counter = 0;

    for v in xmas_vec_horizontal {
        counter += xmas_count(&mut v.iter())?;
        counter += xmas_count(&mut v.iter().rev())?;
    }
    for v in &xmas_vec_vertical {
        counter += xmas_count(&mut v.iter())?;
        counter += xmas_count(&mut v.iter().rev())?;
    }
    for v in &xmas_vec_diagonal_left_to_right {
        counter += xmas_count(&mut v.iter())?;
        counter += xmas_count(&mut v.iter().rev())?;
    }
    for v in &xmas_vec_diagonal_right_to_left {
        counter += xmas_count(&mut v.iter())?;
        counter += xmas_count(&mut v.iter().rev())?;
    }

    Ok(counter)
}

fn puzzle02<P>(filename: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    let mut xmas_vec_horizontal: Vec<Vec<XMAS>> = Vec::new();
    for line in buf.lines().filter_map(|x| x.ok()) {
        let xmas_inner_vec: Result<Vec<XMAS>, _> =
            line.chars().map(|c| XMAS::try_from(c)).collect();
        xmas_vec_horizontal.push(xmas_inner_vec?);
    }

    let rows = xmas_vec_horizontal.len();
    let columns = xmas_vec_horizontal
        .get(0)
        .context("not enough columns")?
        .len();

    let mut counter = 0
    for i in 1..rows - 1 {
        for j in 1..columns - 1 {
            
        }
    }

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

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 4)
    }
}
