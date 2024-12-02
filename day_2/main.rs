#![allow(dead_code)]
extern crate core;

use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn _line_is_safe(mut parts: VecDeque<i64>) -> bool {
    let mut prev = parts.pop_front().expect("At least one");

    let desc = prev > parts[0];

    let range = 1..=3;

    for rest in parts {
        if (prev > rest) != desc {
            return false;
        }
        let delta = (prev - rest).abs();
        if !range.contains(&delta) {
            return false;
        }

        prev = rest;
    }

    true
}

async fn line_is_safe(line: String) -> bool {
    let parts: VecDeque<_> = line
        .split(" ")
        .map(|it| it.parse::<i64>().unwrap_or_default())
        .collect();

    _line_is_safe(parts)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;
    for line in file.map_while(Result::ok) {
        if line_is_safe(line).await {
            result += 1;
        }
    }
    Ok(result)
}

async fn line_is_safe_advanced(line: String) -> bool {
    let parts: VecDeque<_> = line
        .split(" ")
        .map(|it| it.parse::<i64>().unwrap_or_default())
        .collect();

    for idx in 0..parts.len() {
        let mut updated = parts.clone();
        updated.remove(idx);

        if _line_is_safe(updated) {
            return true;
        }
    }

    false
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;
    for line in file.map_while(Result::ok) {
        if line_is_safe_advanced(line).await {
            result += 1;
        }
    }
    Ok(result)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::test]
async fn test_simple_minimal() {
    let answer = 2;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 479;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 14;

    let file = read_lines("advanced.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 531;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read advanced.txt");

    let result = advanced(file).await.expect("Oof 2");

    dbg!(result);
    assert!(result > 527);
    assert_eq!(result, answer);
}
