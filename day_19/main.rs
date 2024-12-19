#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// track possibility of goal given previous:
type Memo = HashMap<(String, String), bool>;

fn matches<S: AsRef<str>>(goal: &str, previous: &str, patterns: &[S], memo: &mut Memo) -> bool {
    if goal.is_empty() {
        return true;
    }

    for candidate in patterns {
        // None if it doesn't start with candiate, Some if it does:
        let candidate_str = candidate.as_ref();

        if let Some(subpattern) = goal.strip_prefix(candidate_str) {
            let cache_key = (subpattern.to_string(), previous.to_string());

            if let Some(existing) = memo.get(&cache_key) {
                return *existing;
            }

            // recurse here:
            if matches(subpattern, candidate_str, patterns, memo) {
                // stop after finding one
                memo.insert(cache_key, true);
                return true;
            } else {
                memo.insert(cache_key, false);
            }
        }
    }

    false
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut pattern_mode = true;

    let mut patterns = vec![];
    let mut goals = vec![];

    for line in file.map_while(Result::ok) {
        if line.is_empty() {
            pattern_mode = false;
        } else if pattern_mode {
            for pattern in line.split(",") {
                patterns.push(pattern.trim().to_string())
            }
        } else {
            goals.push(line)
        }
    }

    let mut result = 0;

    for (idx, goal) in goals.iter().enumerate() {
        eprintln!("{} / {}", idx + 1, goals.len());

        // reset memo per run:
        let mut memo = Memo::new();

        if matches(goal, "", &patterns, &mut memo) {
            result += 1;
        }
    }

    Ok(result)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    for line in file.map_while(Result::ok) {
        println!("{}", line);
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
async fn test_simple_minimal() {
    let answer = 6;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 344;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    let result = simple(file).await.expect("Oof 1");

    assert!(result > 341, "too low :( ");

    assert_eq!(result, answer);
}

// #[tokio::test]
// async fn test_advanced_minimal() {
//     let answer = 0;
//
//     let file = read_lines("minimal.txt")
//         .await
//         .expect("Should be able to read minimal.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_advanced() {
//     let answer = 0;
//
//     let file = read_lines("input.txt")
//         .await
//         .expect("Should be able to read input.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 2"), answer);
// }
