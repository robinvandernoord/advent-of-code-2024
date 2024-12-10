#![allow(dead_code)]

use std::collections::{BTreeMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Point = (i64, i64);

type Matrix = BTreeMap<Point, i64>;

#[must_use]
fn build_matrix(file: FileHandle) -> (Matrix, Vec<Point>) {
    let mut matrix: Matrix = BTreeMap::new();
    let mut trailheads = vec![];

    for (x, line) in file.map_while(Result::ok).enumerate() {
        for (y, char) in line.chars().enumerate() {
            let number = char.to_digit(10).map(|it| it as i64).unwrap_or(-1);
            let point = (x as i64, y as i64);
            matrix.insert(point, number);

            if number == 0 {
                trailheads.push(point);
            }
        }
    }

    (matrix, trailheads)
}

fn walk_trail(position: &Point, height: i64, matrix: &Matrix, found: &mut HashSet<Point>) {
    if height == 9 {
        found.insert(*position);
        return;
    }
    let target = &(height + 1);

    // try 4 directions
    let left = (position.0, position.1 - 1);
    if matrix.get(&left).unwrap_or(&-1) == target {
        walk_trail(&left, *target, matrix, found);
    };
    let right = (position.0, position.1 + 1);
    if matrix.get(&right).unwrap_or(&-1) == target {
        walk_trail(&right, *target, matrix, found);
    }
    let up = (position.0 - 1, position.1);
    if matrix.get(&up).unwrap_or(&-1) == target {
        walk_trail(&up, *target, matrix, found);
    }
    let down = (position.0 + 1, position.1);
    if matrix.get(&down).unwrap_or(&-1) == target {
        walk_trail(&down, *target, matrix, found);
    }
}

fn score_trailhead(trailhead: &Point, matrix: &Matrix) -> i64 {
    // score determined by amount of 9 points reachable
    assert_eq!(matrix[trailhead], 0);

    let mut positions: HashSet<Point> = Default::default();

    walk_trail(trailhead, 0, matrix, &mut positions);
    positions.len() as i64
}

fn point_to_string(point: &Point) -> String {
    format!("{}.{};", point.0, point.1)
}

fn walk_trail_tracked(
    position: &Point,
    height: i64,
    matrix: &Matrix,
    current_route: String,
    found: &mut HashSet<String>,
) {
    let route = format!("{};{}", current_route, point_to_string(position));

    if height == 9 {
        found.insert(route);
        return;
    }
    let target = &(height + 1);

    // try 4 directions
    let left = (position.0, position.1 - 1);
    if matrix.get(&left).unwrap_or(&-1) == target {
        walk_trail_tracked(&left, *target, matrix, route.clone(), found);
    };
    let right = (position.0, position.1 + 1);
    if matrix.get(&right).unwrap_or(&-1) == target {
        walk_trail_tracked(&right, *target, matrix, route.clone(), found);
    }
    let up = (position.0 - 1, position.1);
    if matrix.get(&up).unwrap_or(&-1) == target {
        walk_trail_tracked(&up, *target, matrix, route.clone(), found);
    }
    let down = (position.0 + 1, position.1);
    if matrix.get(&down).unwrap_or(&-1) == target {
        walk_trail_tracked(&down, *target, matrix, route.clone(), found);
    }
}

fn rate_trailhead(trailhead: &Point, matrix: &Matrix) -> i64 {
    // rating determined by amount of unique hiking paths
    let mut positions: HashSet<String> = Default::default();

    walk_trail_tracked(trailhead, 0, matrix, String::new(), &mut positions);
    positions.len() as i64
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let (matrix, trailheads) = build_matrix(file);

    let result = trailheads
        .iter()
        .map(|th| score_trailhead(th, &matrix))
        .sum();

    Ok(result)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let (matrix, trailheads) = build_matrix(file);
    let result = trailheads
        .iter()
        .map(|th| rate_trailhead(th, &matrix))
        .sum();

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
    let answer = 36;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 607;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 81;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 1384;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
