#![allow(dead_code)]

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type FlatDiskMap = Vec<i64>;
type DiskMap = Vec<Option<i64>>;

fn parse_disk_map(line: &str) -> DiskMap {
    let mut disk_map: DiskMap = vec![];
    let mut idx = 0;

    for (i, char) in line.chars().enumerate() {
        let is_data = i % 2 == 0;
        let value = char.to_digit(10).expect("Should be a number") as i64;
        for _ in 0..value {
            if is_data {
                disk_map.push(Some(idx));
            } else {
                disk_map.push(None);
            }
        }
        if is_data {
            idx += 1;
        }
    }

    disk_map
}

fn find_index<T: PartialEq>(vec: &[T], name: &T) -> Option<usize> {
    vec.iter().position(|n| n == name)
}

fn optimize_disk_map(disk_map: &mut DiskMap) {
    while disk_map.contains(&None) {
        let Some(last) = disk_map.pop().flatten() else {
            continue;
        };
        let empty = find_index(disk_map, &None).expect("Already checked if disk_map contains None");
        disk_map[empty] = Some(last);
    }
}

fn calculate_checksum(disk_map: &DiskMap) -> i64 {
    let mut result = 0;
    for (idx, value) in disk_map.iter().enumerate() {
        result += value.unwrap_or(0) * idx as i64
    }

    result
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    if let Some(line) = file.map_while(Result::ok).next() {
        let mut disk_map = parse_disk_map(&line);
        optimize_disk_map(&mut disk_map);
        let checksum = calculate_checksum(&disk_map);
        return Ok(checksum);
    }
    Ok(0)
}

fn find_consecutive<T: PartialEq>(vec: &[T], n: usize, target: T) -> Option<usize> {
    vec.windows(n)
        .position(|window| window.iter().all(|item| *item == target))
}

fn debug_disk_map(disk_map: &DiskMap) {
    let as_char: Vec<_> = disk_map
        .iter()
        .map(|it| match it {
            None => ".".to_string(),
            Some(num) => num.to_string(),
        })
        .collect();

    dbg!(as_char.join(""));
}

fn optimize_disk_map_by_file(disk_map: &mut DiskMap) {
    let mut idx = (disk_map.len() - 1) + 1;
    let mut current: Option<i64> = None;
    let mut current_size = 0;
    let mut seen = HashSet::new();

    while idx > 0 {
        idx -= 1;
        let new = disk_map[idx];

        if new == current {
            current_size += 1;
            continue;
        } else if current.is_some() {
            // end of block!
            let value = current.expect("Already checked if is_some");
            if !seen.contains(&value) {
                seen.insert(value);
                if let Some(free_space_index) = find_consecutive(disk_map, current_size, None) {
                    // found free space!
                    for delta in 0..current_size {
                        if free_space_index < idx {
                            // don't swap further away!
                            disk_map.swap(idx + (delta + 1), free_space_index + delta);
                        }
                    }
                    // debug_disk_map(disk_map);
                }
                // else: not enough free space, ignore
            }
        }
        current = new;
        current_size = 1;
    }
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    if let Some(line) = file.map_while(Result::ok).next() {
        let mut disk_map = parse_disk_map(&line);
        optimize_disk_map_by_file(&mut disk_map);
        let checksum = calculate_checksum(&disk_map);
        return Ok(checksum);
    }
    Ok(0)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::test]
async fn test_simple_very_minimal() {
    let answer = 60;

    let file = read_lines("tiny.txt")
        .await
        .expect("Should be able to read tiny.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple_minimal() {
    let answer = 1928;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 6211348208140;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 2858;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 6239783302560;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
