#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Operation {
    Add,
    Mul,
    Concat,
}

fn concat(n1: i64, n2: i64) -> i64 {
    let mut n3 = n1.to_string();
    n3.push_str(&n2.to_string());

    n3.parse().expect("n1 + n2 should be a number")
}

impl Operation {
    fn apply(&self, left: i64, right: i64) -> i64 {
        match self {
            Self::Add => left + right,
            Self::Mul => left * right,
            Self::Concat => concat(left, right),
        }
    }
}

fn generate_combinations<T: Copy>(elements: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut results = vec![];
    for element in elements {
        let sub_combinations = generate_combinations(elements, n - 1);
        for mut combo in sub_combinations {
            combo.insert(0, *element);
            results.push(combo);
        }
    }
    results
}

fn check_line(line: &str, operations: &[Operation]) -> i64 {
    let mut parts = line.split(": ");

    let sum: i64 = parts
        .next()
        .expect("Should have two parts")
        .parse()
        .expect("Should be a number");
    let numbers: Vec<i64> = parts
        .next()
        .expect("Should have two parts")
        .split(" ")
        .map(|n| n.parse().expect("Should be a number"))
        .collect();

    let combinations = generate_combinations(operations, numbers.len() - 1);
    for combination in combinations {
        let mut numbers_i = numbers.iter();
        let mut total = *numbers_i.next().expect("There must be a first");

        for (idx, number) in numbers_i.enumerate() {
            let operation = combination[idx];
            total = operation.apply(total, *number);
        }

        if total == sum {
            return sum;
        }
    }

    0
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;
    let operations = vec![Operation::Add, Operation::Mul];

    for line in file.map_while(Result::ok) {
        result += check_line(&line, &operations);
    }
    Ok(result)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;
    let operations = vec![Operation::Add, Operation::Mul, Operation::Concat];

    for line in file.map_while(Result::ok) {
        result += check_line(&line, &operations);
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
    let answer = 3749;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 1399219271639;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 11387;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 275791737999003;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
