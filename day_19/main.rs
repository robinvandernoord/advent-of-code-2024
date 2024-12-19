#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// track possibility of goal given previous:
type Memo = HashMap<(String, String), bool>;
type CountMemo = HashMap<(String, String), i64>;

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

fn count_matches<S: AsRef<str>>(
    goal: &str,
    previous: &str,
    patterns: &[S],
    memo: &mut CountMemo,
) -> i64 {
    if goal.is_empty() {
        return 1;
    }

    let mut result = 0;

    for candidate in patterns {
        // None if it doesn't start with candiate, Some if it does:
        let candidate_str = candidate.as_ref();

        if let Some(subpattern) = goal.strip_prefix(candidate_str) {
            let cache_key = (subpattern.to_string(), previous.to_string());

            let subresult: i64;
            if let Some(existing) = memo.get(&cache_key) {
                subresult = *existing;
            } else {
                // recurse here:
                subresult = count_matches(subpattern, candidate_str, patterns, memo);
                memo.insert(cache_key, subresult);
            }

            result += subresult
        }
    }

    result
}

fn parse_file(file: FileHandle) -> (Vec<String>, Vec<String>) {
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

    (goals, patterns)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let (goals, patterns) = parse_file(file);

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
    let (goals, patterns) = parse_file(file);

    let mut result = 0;

    for (idx, goal) in goals.iter().enumerate() {
        eprintln!("{} / {}", idx + 1, goals.len());

        // reset memo per run:
        let mut memo = CountMemo::new();

        result += count_matches(goal, "", &patterns, &mut memo);
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

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 16;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 996172272010026;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
