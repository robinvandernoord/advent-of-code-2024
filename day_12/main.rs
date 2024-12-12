#![allow(dead_code)]

use std::collections::{BTreeMap, HashMap};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

type Point = (i64, i64);
type Matrix = BTreeMap<Point, char>;
type PlotMap = BTreeMap<Point, i64>;

fn draw<D: Display>(plot_map: &BTreeMap<Point, D>) {
    let mut cur_x = 0;

    for (point, idx) in plot_map {
        if point.0 != cur_x {
            println!();
            cur_x = point.0;
        }
        print!("{idx}");
    }
    println!();
}

fn parse_matrix(file: FileHandle) -> Matrix {
    let mut matrix: Matrix = Default::default();
    for (x, line) in file.map_while(Result::ok).enumerate() {
        for (y, plant) in line.chars().enumerate() {
            matrix.insert((x as i64, y as i64), plant);
        }
    }

    matrix
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn mutate(&self, other: &Point) -> Point {
        match self {
            Direction::Up => (other.0 - 1, other.1),
            Direction::Right => (other.0, other.1 + 1),
            Direction::Down => (other.0 + 1, other.1),
            Direction::Left => (other.0, other.1 - 1),
        }
    }

    pub fn get_plant<'mtx>(&self, other: &Point, matrix: &'mtx Matrix) -> &'mtx char {
        let new_point = self.mutate(other);
        matrix.get(&new_point).unwrap_or(&' ')
    }
}

// async fn simple(file: FileHandle) -> anyhow::Result<i64> {
//     let matrix = parse_matrix(file);
//
//     let mut plot_idx: i64 = 0;
//     let mut plot_indices: PlotMap = Default::default();
//
//     let mut area_per_plot_idx: HashMap<i64, i64> = Default::default();
//     let mut perimeter_per_plot_idx: HashMap<i64, i64> = Default::default();
//
//     for (point, plant) in &matrix {
//         // if no plot index yet, assign one.
//         // for every in top, right, bottom, left: check if same char and appoint my plot idx.
//         let my_idx = match plot_indices.get(&point) {
//             None => {
//                 let idx = plot_idx;
//                 plot_indices.insert(*point, idx);
//                 plot_idx += 1;
//                 idx
//             }
//             Some(idx) => {
//                 *idx
//             }
//         };
//
//         let mut perimeter = 4;
//
//         for direction in DIRECTIONS {
//             let other_point = direction.mutate(point);
//             let other_plant = matrix.get(&other_point).unwrap_or(&' ');
//             if plant == other_plant {
//                 plot_indices.insert(other_point, my_idx);
//                 perimeter -= 1;
//             }
//         }
//
//
//         // now update area & perimeter:
//         *area_per_plot_idx.entry(my_idx).or_default() += 1;
//         *perimeter_per_plot_idx.entry(my_idx).or_default() += perimeter;
//
//     }
//
//     Ok(
//         area_per_plot_idx.iter().map(|(region, area)| {
//             let perimeter = perimeter_per_plot_idx[region];
//             perimeter * area
//         }).sum()
//     )
// }

fn walk(point: &Point, plant: &char, plot_idx: &i64, plot_map: &mut PlotMap, matrix: &Matrix) {
    if plot_map.contains_key(point) {
        // already seen
        return;
    }

    plot_map.insert(*point, *plot_idx);
    for direction in DIRECTIONS {
        let new_point = direction.mutate(point);
        let new_plant = matrix.get(&new_point).unwrap_or(&' ');
        if new_plant == plant {
            walk(&new_point, plant, plot_idx, plot_map, matrix);
        }
    }
}


fn assign_ids(matrix: &Matrix) -> PlotMap {
    // for every point in matrix without plot id:
    // continue walking everything with same plant.
    let mut plot_idx: i64 = 0;
    let mut plot_indices: PlotMap = Default::default();
    for (point, plant) in matrix {
        if !plot_indices.contains_key(point) {
            walk(point, plant, &plot_idx, &mut plot_indices, &matrix);
            plot_idx += 1;
        }
    }

    plot_indices
}


async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let matrix = parse_matrix(file);

    // draw(&matrix);
    let plot_indices = assign_ids(&matrix);

    // draw(&plot_indices);

    let mut area_per_plot_idx: HashMap<i64, i64> = Default::default();
    let mut perimeter_per_plot_idx: HashMap<i64, i64> = Default::default();

    // now loop through matrix again and calculate score:

    for (point, plant) in &matrix {
        let my_plot_id = plot_indices[point];

        let mut perimeter = 4;
        for direction in DIRECTIONS {
            let other_plant = direction.get_plant(point, &matrix);
            if plant == other_plant {
                perimeter -= 1;
            }
        }

        *area_per_plot_idx.entry(my_plot_id).or_default() += 1;
        *perimeter_per_plot_idx.entry(my_plot_id).or_default() += perimeter;
    }


    Ok(area_per_plot_idx
        .iter()
        .map(|(region, area)| {
            let perimeter = perimeter_per_plot_idx[region];
            perimeter * area
        })
        .sum())
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    // uses 'number of sides' instead of 'perimeter'
    let matrix = parse_matrix(file);
    let mut plot_idx: i64 = 0;
    let mut plot_indices: PlotMap = Default::default();
    for (point, plant) in &matrix {
        if !plot_indices.contains_key(point) {
            walk(point, plant, &plot_idx, &mut plot_indices, &matrix);
            plot_idx += 1;
        }
    }
    draw(&matrix);
    draw(&plot_indices);
    Ok(0)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #[tokio::test]
// async fn test_simple_minimal_0() {
//     let answer = 0;
//
//     let file = read_lines("minimal0.txt")
//         .await
//         .expect("Should be able to read minimal0.txt");
//
//     assert_eq!(simple(file).await.expect("Oof 1"), answer);
// }

#[tokio::test]
async fn test_simple_minimal_1() {
    let answer = 140;

    let file = read_lines("minimal1.txt")
        .await
        .expect("Should be able to read minimal1.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple_minimal_2() {
    let answer = 772;

    let file = read_lines("minimal2.txt")
        .await
        .expect("Should be able to read minimal2.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple_minimal_3() {
    let answer = 1930;

    let file = read_lines("minimal3.txt")
        .await
        .expect("Should be able to read minimal3.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 1424006;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

// #[tokio::test]
// async fn test_advanced_minimal_1() {
//     let answer = 80;
//
//     let file = read_lines("minimal1.txt")
//         .await
//         .expect("Should be able to read minimal1.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_advanced_minimal_2() {
//     let answer = 436;
//
//     let file = read_lines("minimal2.txt")
//         .await
//         .expect("Should be able to read minimal2.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_advanced_minimal_3() {
//     let answer = 1206;
//
//     let file = read_lines("minimal3.txt")
//         .await
//         .expect("Should be able to read minimal3.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_advanced_minimal_4() {
//     let answer = 236;
//
//     let file = read_lines("minimal4.txt")
//         .await
//         .expect("Should be able to read minimal4.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }
//
// #[tokio::test]
// async fn test_advanced_minimal_5() {
//     let answer = 368;
//
//     let file = read_lines("minimal5.txt")
//         .await
//         .expect("Should be able to read minimal5.txt");
//
//     assert_eq!(advanced(file).await.expect("Oof 1"), answer);
// }

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
