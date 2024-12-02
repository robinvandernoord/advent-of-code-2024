#![allow(dead_code)]
extern crate core;

use core::fmt;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

async fn line_is_safe(line: String) -> bool {
    let mut parts: VecDeque<_> = line
        .split(" ")
        .map(|it| it.parse::<i64>().unwrap_or_default())
        .collect();

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

#[derive(Debug)]
enum DampenerState {
    LifeLeft,
    Dead,
}

#[derive(Debug)]
struct Dampener {
    inner: DampenerState,
}

impl Default for Dampener {
    fn default() -> Self {
        Self {
            inner: DampenerState::LifeLeft,
        }
    }
}

impl Dampener {
    fn activate(&mut self) -> Result<(), OutofLifes> {
        match self.inner {
            DampenerState::LifeLeft => {
                self.inner = DampenerState::Dead;
                Ok(())
            }
            DampenerState::Dead => Err(OutofLifes {}),
        }
    }
}

#[derive(Debug, Clone)]
struct OutofLifes {}
impl fmt::Display for OutofLifes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Out of Lifes!")
    }
}

async fn line_is_safe_with_dampener(line: &str) -> Result<bool, OutofLifes> {
    let mut dampener = Dampener::default();

    let mut parts: VecDeque<_> = line
        .split(" ")
        .map(|it| it.parse::<i64>().unwrap_or_default())
        .collect();

    let range = 1..=3;

    let mut prev = parts.pop_front().expect("At least one");
    // let desc = prev > parts[0];
    // this skips to third if second is invalid:
    let desc = if range.contains(&(prev - parts[0].abs())) {
        prev > parts[0]
    } else if range.contains(&(prev - parts[1].abs())) {
        prev > parts[1]
    } else {
        prev > parts[2]
    };

    for rest in parts {
        if (prev > rest) != desc {
            dampener.activate()?;
            continue;
        }
        let delta = (prev - rest).abs();
        if !range.contains(&delta) {
            dampener.activate()?;
            continue;
        }

        prev = rest;
    }

    Ok(true)
}

async fn line_is_safe_skip_first(line: &str) -> bool {
    let mut parts: VecDeque<_> = line
        .split(" ")
        .map(|it| it.parse::<i64>().unwrap_or_default())
        .collect();

    parts.pop_front().expect("At least one");
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

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;
    for line in file.map_while(Result::ok) {
        if line_is_safe(line).await {
            result += 1;
        }
    }
    Ok(result)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;
    for line in file.map_while(Result::ok) {
        // dampener only works after the first item, so try again without dampener:
        let valid = line_is_safe_with_dampener(&line).await.unwrap_or(false)
            || line_is_safe_skip_first(&line).await;
        if valid {
            result += 1;
        } else {
            dbg!(line);
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
    let answer = 0;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read advanced.txt");

    let result = advanced(file).await.expect("Oof 2");

    dbg!(result);
    assert!(result > 527);
    assert_eq!(result, answer);
}
