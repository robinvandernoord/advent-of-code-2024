#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn generate_combinations<T: Copy + PartialEq>(elements: &[T], n: usize) -> Vec<Vec<T>> {
    if n == 0 {
        return vec![vec![]];
    }

    let mut results = vec![];
    for &element in elements.iter() {
        let sub_combinations = generate_combinations(elements, n - 1);
        for mut combo in sub_combinations {
            if combo.is_empty() || combo[0] != element {
                combo.insert(0, element);
                results.push(combo);
            }
        }
    }
    results
}

type Point = (i64, i64);
type Matrix = HashMap<Point, char>;
type Antennae = HashMap<char, Vec<Point>>;

fn parse_matrix(file: FileHandle) -> (Matrix, Antennae) {
    let mut matrix: Matrix = Default::default();
    let mut antennae: Antennae = Default::default();

    for (x, line) in file.map_while(Result::ok).enumerate() {
        for (y, char) in line.chars().enumerate() {
            let point: Point = (x as i64, y as i64);
            matrix.insert(point, char);

            if char != '.' {
                antennae.entry(char).or_default().push(point);
            }
        }
    }

    (matrix, antennae)
}

fn find_antinode(p1: &Point, p2: &Point) -> Point {
    let d_x = p2.0 - p1.0;
    let d_y = p2.1 - p1.1;

    (p1.0 - d_x, p1.1 - d_y)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let (matrix, antennae) = parse_matrix(file);

    // hashset to prevent duplicates:
    let mut antinodes: HashSet<Point> = Default::default();

    for points in antennae.values() {
        for combination in generate_combinations(points, 2) {
            let mut combi_i = combination.iter();
            let first = combi_i.next().expect("Should have 2 points");
            let second = combi_i.next().expect("Should have 2 points");

            let antinode = find_antinode(first, second);
            if matrix.contains_key(&antinode) {
                antinodes.insert(antinode);
            }
        }
    }

    Ok(antinodes.len() as i64)
}

fn find_antinodes(p1: &Point, p2: &Point, matrix: &Matrix) -> HashSet<Point> {
    let mut nodes = HashSet::new();

    let d_x = p2.0 - p1.0;
    let d_y = p2.1 - p1.1;

    for mul in 0..10_000 {
        let p3 = (p1.0 - d_x * mul, p1.1 - d_y * mul);
        if !matrix.contains_key(&p3) {
            // out of bounds, stop!
            break;
        }
        nodes.insert(p3);
    }

    nodes
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let (matrix, antennae) = parse_matrix(file);

    // hashset to prevent duplicates:
    let mut antinodes: HashSet<Point> = Default::default();

    for points in antennae.values() {
        for combination in generate_combinations(points, 2) {
            let mut combi_i = combination.iter();
            let first = combi_i.next().expect("Should have 2 points");
            let second = combi_i.next().expect("Should have 2 points");
            antinodes.extend(find_antinodes(first, second, &matrix));

            dbg!(first, second);
        }
    }
    Ok(antinodes.len() as i64)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #[tokio::test]
// async fn test_simple_very_minimal() {
//     let answer = 2;
//
//     let file = read_lines("tiny.txt")
//         .await
//         .expect("Should be able to read tiny.txt");
//
//     assert_eq!(simple(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_simple_minimal() {
//     let answer = 14;
//
//     let file = read_lines("minimal.txt")
//         .await
//         .expect("Should be able to read minimal.txt");
//
//     assert_eq!(simple(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_simple() {
//     let answer = 367;
//
//     let file = read_lines("input.txt")
//         .await
//         .expect("Should be able to read input.txt");
//
//     assert_eq!(simple(file).await.expect("Oof 1"), answer);
// }

#[tokio::test]
async fn test_advanced_very_minimal() {
    let answer = 9;

    let file = read_lines("advanced.txt")
        .await
        .expect("Should be able to read advanced.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 34;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 1285;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
