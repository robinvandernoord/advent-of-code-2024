#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::BitXor;
use std::path::Path;

fn prune(secret: i64) -> i64 {
    secret.rem_euclid(16777216)
}

fn mix(value: i64, secret: i64) -> i64 {
    // To mix a value into the secret number, calculate the bitwise XOR of the given value and the secret number.
    // Then, the secret number becomes the result of that operation.
    value.bitxor(secret)
}

fn calculate_next(initial: i64) -> i64 {
    let mut secret = initial;

    // Calculate the result of multiplying the secret number by 64. Then, mix this result into the secret number. Finally, prune the secret number.
    secret = prune(mix(secret * 64, secret));

    // Calculate the result of dividing the secret number by 32. Round the result down to the nearest integer. Then, mix this result into the secret number. Finally, prune the secret number.
    secret = prune(mix(secret / 32, secret));

    // Calculate the result of multiplying the secret number by 2048. Then, mix this result into the secret number. Finally, prune the secret number.
    secret = prune(mix(secret * 2048, secret));

    secret
}

fn next_secret(initial: i64, times: i64) -> i64 {
    let mut next = initial;

    for _ in 0..times {
        next = calculate_next(next);
    }

    next
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut result = 0;

    for line in file.map_while(Result::ok) {
        let secret: i64 = line.parse().expect("Should be valid number");

        result += next_secret(secret, 2000);
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
async fn test_mix() {
    assert_eq!(mix(15, 42), 37)
}

#[tokio::test]
async fn test_prune() {
    assert_eq!(prune(100000000), 16113920)
}

#[tokio::test]
async fn test_secret_numbers() {
    assert_eq!(next_secret(123, 0), 123);
    assert_eq!(next_secret(123, 1), 15887950);
    assert_eq!(next_secret(123, 2), 16495136);
    assert_eq!(next_secret(123, 3), 527345);
    assert_eq!(next_secret(123, 4), 704524);
    assert_eq!(next_secret(123, 5), 1553684);
    assert_eq!(next_secret(123, 6), 12683156);
    assert_eq!(next_secret(123, 7), 11100544);
    assert_eq!(next_secret(123, 8), 12249484);
    assert_eq!(next_secret(123, 9), 7753432);
    assert_eq!(next_secret(123, 10), 5908254);
}

#[tokio::test]
async fn test_simple_minimal() {
    let answer = 37327623;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 0;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
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
