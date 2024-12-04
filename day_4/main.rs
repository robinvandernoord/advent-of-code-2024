#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Coordinate = (isize, isize);
type Matrix = HashMap<Coordinate, char>;

const TARGET: &str = "XMAS";

fn char_at<'matrix>(matrix: &'matrix Matrix, coord: &Coordinate) -> &'matrix char {
    matrix.get(coord).unwrap_or(&'_')
}

fn try_find_xmas(coordinate: &Coordinate, matrix: &Matrix, d_x: isize, d_y: isize) -> bool {
    let mut coord = *coordinate;

    for char in TARGET.chars() {
        if char_at(matrix, &coord) != &char {
            return false;
        }

        coord = (coord.0 + d_x, coord.1 + d_y)
    }

    true
}

fn try_find_mas(coordinate: &Coordinate, matrix: &Matrix, d_x: isize, d_y: isize) -> bool {
    // 1 check that coord is A
    // 2 check that coord +dx,dy = M
    // 3 check that coord -dx,dy = S
    let mut coord = *coordinate;

    // 1
    if char_at(matrix, &coord) != &'A' {
        return false;
    }

    // 2
    coord = (coordinate.0 + d_x, coordinate.1 + d_y);
    if char_at(matrix, &coord) != &'M' {
        return false;
    }

    // 3
    coord = (coordinate.0 - d_x, coordinate.1 - d_y);
    if char_at(matrix, &coord) != &'S' {
        return false;
    }

    true
}

fn try_find_mas_x(coordinate: &Coordinate, matrix: &Matrix) -> bool {
    // find MAS on one diagonal AND on the other
    let mas_1 = try_find_mas(coordinate, matrix, -1, -1) | try_find_mas(coordinate, matrix, 1, 1);
    let mas_2 = try_find_mas(coordinate, matrix, -1, 1) | try_find_mas(coordinate, matrix, 1, -1);

    // find MAS on horizontal and vertical:
    // let mas_3 = try_find_mas(coordinate, matrix, -1, 0) | try_find_mas(coordinate, matrix, 1, 0);
    // let mas_4 = try_find_mas(coordinate, matrix, 0, -1) | try_find_mas(coordinate, matrix, 0, 1);

    mas_1 & mas_2 // | (mas_3 & mas_4)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut x_es: Vec<Coordinate> = vec![];
    let mut matrix: Matrix = HashMap::new();

    for (x_idx, line) in file.map_while(Result::ok).enumerate() {
        for (y_index, char) in line.chars().enumerate() {
            let key = (x_idx as isize, y_index as isize);
            matrix.insert(key, char);
            if char == 'X' {
                x_es.push(key);
            }
        }
    }

    let mut score = 0;
    for coordinate in &x_es {
        for d_x in -1..=1 {
            for d_y in -1..=1 {
                if try_find_xmas(coordinate, &matrix, d_x, d_y) {
                    score += 1
                }
            }
        }
    }

    Ok(score)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let mut a_coords: Vec<Coordinate> = vec![];
    let mut matrix: Matrix = HashMap::new();

    for (x_idx, line) in file.map_while(Result::ok).enumerate() {
        for (y_index, char) in line.chars().enumerate() {
            let key = (x_idx as isize, y_index as isize);
            matrix.insert(key, char);
            if char == 'A' {
                a_coords.push(key);
            }
        }
    }

    let mut score = 0;
    for coordinate in &a_coords {
        if try_find_mas_x(coordinate, &matrix) {
            score += 1
        }
    }

    Ok(score)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[tokio::test]
async fn test_simple_minimal() {
    let answer = 18;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 2390;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 9;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

// #[tokio::test]
// async fn test_advanced_shape2() {
//     // with MAS in a + shape instead of x
//     let answer = 1;
//
//     let file = read_lines("shape_two.txt")
//         .await
//         .expect("Should be able to read minimal.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }

#[tokio::test]
async fn test_advanced() {
    let answer = 1809;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read advanced.txt");

    let result = advanced(file).await.expect("Oof 2");

    dbg!(result);

    assert!(result > 441); // 441 too low
    assert!(result < 1840); // 1840 too high

    assert_eq!(result, answer);
}
