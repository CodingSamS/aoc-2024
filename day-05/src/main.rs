use anyhow::Context;
use regex::Regex;
use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn puzzle01<P>(filename_rules: P, filename_update: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename_rules)?;
    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)\|(\d+)")?;
    let mut invalid_successors: HashMap<u16, Vec<u16>> = HashMap::new();

    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let capture = line_regex.captures(&line).context("no line capture")?;
        let lower: u16 = capture[1].parse()?;
        let higher: u16 = capture[2].parse()?;
        if invalid_successors.contains_key(&higher) {
            invalid_successors.get_mut(&higher).unwrap().push(lower);
        } else {
            invalid_successors.insert(higher, vec![lower]);
        }
    }

    let file = File::open(filename_update)?;
    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)")?;
    let mut updates: Vec<Vec<u16>> = Vec::new();
    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let mut update: Vec<u16> = Vec::new();
        for capture in line_regex.captures_iter(&line) {
            update.push(capture[0].parse()?)
        }
        updates.push(update);
    }

    let mut result_counter = 0;
    'outer: for update in updates {
        for i in 0..update.len() - 1 {
            let val = update[i];
            for invalid_successor in invalid_successors.get(&val).unwrap_or(&Vec::new()) {
                if update[i + 1..].contains(invalid_successor) {
                    continue 'outer;
                }
            }
        }
        result_counter += i64::from(update[(update.len() - 1) / 2]);
    }

    Ok(result_counter)
}

fn puzzle02<P>(filename_rules: P, filename_update: P) -> anyhow::Result<i64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename_rules)?;
    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)\|(\d+)")?;
    let mut invalid_successors: HashMap<u16, Vec<u16>> = HashMap::new();

    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let capture = line_regex.captures(&line).context("no line capture")?;
        let lower: u16 = capture[1].parse()?;
        let higher: u16 = capture[2].parse()?;
        if invalid_successors.contains_key(&higher) {
            invalid_successors.get_mut(&higher).unwrap().push(lower);
        } else {
            invalid_successors.insert(higher, vec![lower]);
        }
    }

    let file = File::open(filename_update)?;
    let buf = io::BufReader::new(file);

    let line_regex = Regex::new(r"(\d+)")?;
    let mut updates: Vec<Vec<u16>> = Vec::new();
    for line in buf.lines().filter_map(|line_result| line_result.ok()) {
        let mut update: Vec<u16> = Vec::new();
        for capture in line_regex.captures_iter(&line) {
            update.push(capture[0].parse()?)
        }
        updates.push(update);
    }

    let mut result_counter = 0;
    for mut update in updates {
        let mut i = 0;
        let mut is_relevant = false;
        'while_loop: while i < update.len() - 1 {
            let val = update[i];
            if let Some(invalid_successors_val) = invalid_successors.get(&val) {
                for j in i + 1..update.len() {
                    if invalid_successors_val.contains(&update[j]) {
                        is_relevant = true;
                        update.swap(i, j);
                        i = 0;
                        continue 'while_loop;
                    }
                }
            }
            i += 1;
        }
        if is_relevant {
            result_counter += i64::from(update[(update.len() - 1) / 2]);
        }
    }

    Ok(result_counter)
}

fn main() {
    println!(
        "Solution 1: {}",
        puzzle01("data/data_rules_1", "data/data_update_1").unwrap()
    );
    println!(
        "Solution 2: {}",
        puzzle02("data/data_rules_1", "data/data_update_1").unwrap()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {
        assert_eq!(
            crate::puzzle01("data/test_data_rules_1", "data/test_data_update_1").unwrap(),
            143
        )
    }
    #[test]
    fn test2() {
        assert_eq!(
            crate::puzzle02("data/test_data_rules_1", "data/test_data_update_1").unwrap(),
            123
        )
    }
}
