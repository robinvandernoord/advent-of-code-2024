use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn parse_lines(file: FileHandle) -> anyhow::Result<(Vec<i64>, Vec<i64>)> {
    let mut left = vec![];
    let mut right = vec![];

    for line in file.flatten() {
        let parts: Vec<_> = line.split(" ").collect();
        let left_str = parts.first().expect("Should have");
        let right_str = parts.last().expect("Should have");

        let left_i: i64 = left_str.parse()?;
        let right_i: i64 = right_str.parse()?;

        left.push(left_i);
        right.push(right_i);
    }

    Ok((left, right))
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let (mut left, mut right) = parse_lines(file)?;

    left.sort_unstable();
    // left.dedup();

    right.sort_unstable();
    // right.dedup();

    let mut result = 0;
    for (l, r) in left.iter().zip(right) {
        let delta = (l - r).abs();
        result += delta;
    }


    Ok(result)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let (left, right) = parse_lines(file)?;
    let right_counts = right.iter().fold(HashMap::new(), |mut acc, &num| {
        *acc.entry(num).or_insert(0) += 1;
        acc
    });

    let mut result = 0;
    for number in left.iter() {
        let multiplier = right_counts.get(number).unwrap_or(&0);
        result += number * multiplier;
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
async fn test_minimal_simple() {
    let answer = 11;

    let file = read_lines("minimal.txt").await.expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 0"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 2375403;

    let file = read_lines("simple.txt").await.expect("Should be able to read simple.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}


#[tokio::test]
async fn test_minimal_advanced() {
    let answer = 31;

    let file = read_lines("minimal.txt").await.expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 0"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 23082277;

    let file = read_lines("simple.txt").await.expect("Should be able to read advanced.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
