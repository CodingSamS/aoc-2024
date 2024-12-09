use anyhow::bail;
use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, BufRead, Read},
    path::Path,
};

use anyhow::Context;

fn puzzle01<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut buf = io::BufReader::new(file);
    let mut line = String::new();
    buf.read_line(&mut line)?;

    let mut file_id = 0;
    let mut is_file = true;
    let mut disk: Vec<Option<usize>> = Vec::new();
    for c in line
        .strip_suffix("\n")
        .context("does not end with newline")?
        .chars()
    {
        let count = c.to_digit(10).context("no valid digit")?;
        if is_file {
            for _ in 0..count {
                disk.push(Some(file_id));
            }
            file_id += 1;
            is_file = false
        } else {
            for _ in 0..count {
                disk.push(None);
            }
            is_file = true
        }
    }

    for i in 0..disk.len() {
        match disk.get(i) {
            Some(val) => {
                if val == &None {
                    for j in (i + 1..disk.len()).rev() {
                        if disk[j].is_some() {
                            disk.swap(i, j);
                            break;
                        }
                    }
                }
            }
            None => break,
        }
    }

    Ok(disk
        .iter()
        .enumerate()
        .filter_map(|(i, val)| match val {
            Some(v) => Some(i * v),
            None => None,
        })
        .sum())
}

#[derive(Debug)]
enum Block {
    Empty(u32),          // length
    Moved(usize, u32),   // (file_id, length)
    Unmoved(usize, u32), // (file_id, length)
}

fn puzzle02<P>(filename: P) -> anyhow::Result<usize>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let mut buf = io::BufReader::new(file);

    let mut line = String::new();
    buf.read_line(&mut line)?;

    let mut file_id = 0;
    let mut is_file = true;
    let mut disk: Vec<Block> = Vec::new();
    for c in line
        .strip_suffix("\n")
        .context("does not end with newline")?
        .chars()
    {
        let count = c.to_digit(10).context("no valid digit")?;
        if is_file {
            if 0 < count {
                disk.push(Block::Unmoved(file_id, count));
            }
            file_id += 1;
            is_file = false
        } else {
            if 0 < count {
                disk.push(Block::Empty(count));
            }
            is_file = true
        }
    }

    let mut new_disk = Vec::new();
    loop {
        match disk.pop() {
            Some(block) => match block {
                Block::Unmoved(file_id, length) => {
                    let index_free_block = {
                        let mut result = None;
                        for (index, block) in disk.iter().enumerate() {
                            match block {
                                Block::Empty(empty_len) => {
                                    if &length <= empty_len {
                                        result = Some(index);
                                        break;
                                    }
                                }
                                _ => continue,
                            }
                        }
                        result
                    };
                    match index_free_block {
                        Some(index) => {
                            let empty_block = disk.remove(index);
                            match empty_block {
                                Block::Empty(empty_len) => {
                                    let new_empty_len = empty_len - length;
                                    if 0 < new_empty_len {
                                        disk.insert(index, Block::Empty(new_empty_len));
                                    }
                                    disk.insert(index, Block::Moved(file_id, length));
                                    disk.push(Block::Empty(length));
                                }
                                _ => bail!("There should be an empty element on that position"),
                            }
                        }
                        None => disk.push(Block::Moved(file_id, length)),
                    }
                }
                Block::Moved(file_id, length) => {
                    for _ in 0..length {
                        new_disk.push(file_id);
                    }
                }
                Block::Empty(length) => {
                    for _ in 0..length {
                        new_disk.push(0);
                    }
                }
            },
            None => break,
        }
    }

    Ok(new_disk
        .iter()
        .rev()
        .enumerate()
        .map(|(index, val)| index * val)
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
        assert_eq!(crate::puzzle01("data/test_data_1").unwrap(), 1928)
    }

    #[test]
    fn test2() {
        assert_eq!(crate::puzzle02("data/test_data_1").unwrap(), 2858)
    }
}
