#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn next(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

type Location = (i64, i64);
type Matrix = HashMap<Location, char>;
type Guard = Location;

fn walk_route(mut guard: Guard, matrix: &Matrix, points_visited: &mut HashSet<Location>) -> bool {
    let mut direction = Direction::Up;
    let mut points_visited_with_direction: HashSet<(Location, Direction)> = HashSet::new();

    loop {
        points_visited.insert(guard);

        if points_visited_with_direction.contains(&(guard, direction)) {
            // loop detected!
            return true;
        }

        points_visited_with_direction.insert((guard, direction));

        let next_location: Location = match direction {
            Direction::Up => (guard.0 - 1, guard.1),
            Direction::Right => (guard.0, guard.1 + 1),
            Direction::Down => (guard.0 + 1, guard.1),
            Direction::Left => (guard.0, guard.1 - 1),
        };

        let point = matrix.get(&next_location);

        match point {
            None => {
                // out of bounds, stop!
                return false;
            }
            Some('#') => {
                // turn
                direction = direction.next();
            }
            Some(_) => {
                // . or ^, just walk
                guard = next_location;
            }
        }
    }
}

fn parse_matrix(file: FileHandle) -> (Guard, Matrix) {
    let mut matrix = Matrix::new();
    let mut guard: Guard = (0, 0);

    for (y, line) in file.map_while(Result::ok).enumerate() {
        for (x, char) in line.chars().enumerate() {
            let x = x as i64;
            let y = y as i64;

            if char == '^' {
                guard = (y, x)
            }

            matrix.insert((y, x), char);
        }
    }

    (guard, matrix)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let (guard, matrix) = parse_matrix(file);

    let mut points_visited = HashSet::new();
    let loop_detected = walk_route(guard, &matrix, &mut points_visited);

    if loop_detected {
        panic!("This is only supposed to happen in part 2!")
    }

    Ok(points_visited.len() as i64)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let (guard, matrix) = parse_matrix(file);

    let mut loops = 0;
    for (point, current_char) in &matrix {
        // eprintln!("{} / {}; {} found!", idx, matrix.len(), loops);
        if current_char != &'.' {
            // irrelevant, skip!
            continue;
        }
        let mut matrix_with_obstruction = matrix.clone();
        matrix_with_obstruction.insert(*point, '#');

        if walk_route(guard, &matrix_with_obstruction, &mut HashSet::new()) {
            // loop detected!
            loops += 1;
        }
    }

    Ok(loops)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::test]
async fn test_simple_minimal() {
    let answer = 41;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 4982;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 6;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 1663;

    // note: run with `cargo test --release` or it will take ages!

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
