#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let re = regex::Regex::new(r"mul\((\d+?,\d+?)\)")?;

    let mut lines = String::new();
    for line in file.map_while(Result::ok) {
        lines.push_str(&line);
    }

    let mut result = 0;
    for (_, [tuple]) in re.captures_iter(&lines).map(|c| c.extract()) {
        let mut parts = tuple.split(",");

        let left = parts.next().expect("Should have two numeric parts");
        let right = parts.next().expect("Should have two numeric parts");

        let left_no: i64 = left.parse().expect("Should have two numeric parts");
        let right_no: i64 = right.parse().expect("Should have two numeric parts");

        result += left_no * right_no;
    }

    Ok(result)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let re = regex::Regex::new(r"mul\((\d+?,\d+?)\)")?;

    let mut lines = String::new();
    for line in file.map_while(Result::ok) {
        lines.push_str(&line);
    }

    // split lines on 'do'
    // throw away regions that start with n't
    // join again

    let parts = lines.split("do");

    let filtered: Vec<_> = parts.filter(|it| !it.starts_with("n't")).collect();
    let lines = filtered.join(" ");

    let mut result = 0;
    for (_, [tuple]) in re.captures_iter(&lines).map(|c| c.extract()) {
        let mut parts = tuple.split(",");

        let left = parts.next().expect("Should have two numeric parts");
        let right = parts.next().expect("Should have two numeric parts");

        let left_no: i64 = left.parse().expect("Should have two numeric parts");
        let right_no: i64 = right.parse().expect("Should have two numeric parts");

        result += left_no * right_no;
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
    let answer = 161;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 192767529;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 48;

    let file = read_lines("advanced.txt")
        .await
        .expect("Should be able to read advanced.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 104083373;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
