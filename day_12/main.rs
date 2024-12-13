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

fn parse_matrix_file(file: FileHandle) -> Matrix {
    let mut matrix: Matrix = Default::default();
    for (x, line) in file.map_while(Result::ok).enumerate() {
        for (y, plant) in line.chars().enumerate() {
            matrix.insert((x as i64, y as i64), plant);
        }
    }

    matrix
}

fn parse_matrix_str(rows: &str) -> Matrix {
    let mut matrix: Matrix = Default::default();
    for (x, line) in rows.lines().enumerate() {
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
}

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

fn plotmap_from_file(file: FileHandle) -> PlotMap {
    let matrix = parse_matrix_file(file);
    assign_ids(&matrix)
}

fn plotmap_from_string(input: &str) -> PlotMap {
    let matrix = parse_matrix_str(input);
    assign_ids(&matrix)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let plots = plotmap_from_file(file);

    // draw(&plots);

    let mut area_per_plot_idx: HashMap<i64, i64> = Default::default();
    let mut perimeter_per_plot_idx: HashMap<i64, i64> = Default::default();

    // now loop through matrix again and calculate score:

    for (point, my_plot) in &plots {
        let mut perimeter = 4;
        for direction in DIRECTIONS {
            let other_point = direction.mutate(point);
            let other_plot = plots.get(&other_point).unwrap_or(&-1);

            if my_plot == other_plot {
                perimeter -= 1;
            }
        }

        *area_per_plot_idx.entry(*my_plot).or_default() += 1;
        *perimeter_per_plot_idx.entry(*my_plot).or_default() += perimeter;
    }

    Ok(area_per_plot_idx
        .iter()
        .map(|(region, area)| {
            let perimeter = perimeter_per_plot_idx[region];
            perimeter * area
        })
        .sum())
}

fn count_neighbors(point: &Point, plot_idx: &i64, plots: &PlotMap) -> i64 {
    let mut neighbors = 0;
    for direction in DIRECTIONS {
        let other_point = direction.mutate(point);
        let other_plot = plots.get(&other_point).unwrap_or(&-1);
        if other_plot == plot_idx {
            neighbors += 1;
        }
    }

    neighbors
}

/// O is empty space or other type
/// ? is current block
/// X is related type
/// Don't forget other orientations (e.g. vertical if example is horizontal)!!
enum Shape {
    /// 0 neighbors
    /// OOO
    /// O?O
    /// OOO
    Single, // 4 corners per single

    /// 1 neighbor
    /// OOO0000
    /// O?XXX?0
    /// OOO0000
    LineEdge, // 2 corners per edge

    /// 2 neighbors
    /// 000
    /// X?X
    /// 000
    Middle, // 0 corners

    /// 2 neighbors
    /// 000
    /// 0?X
    /// 0X0
    CornerEdge(u8), // 2 or 1 corners depending on bottom right diagonal

    /// 3 neighbors
    /// 0X0
    /// X?X
    /// 000
    TShape(u8), // 2 corners (but could be 1 or 0 if diagonals are X)

    /// 4 neighbors
    /// XXX
    /// X?X
    /// XXX
    Center, // 0 corners
}

fn count_corners(point: &Point, plot_idx: &i64, plots: &PlotMap) -> i64 {
    let point_left = Direction::Left.mutate(point);
    let value_left = plots.get(&point_left).unwrap_or(&-1);
    let left_is_neighbor = value_left == plot_idx;

    let point_right = Direction::Right.mutate(point);
    let value_right = plots.get(&point_right).unwrap_or(&-1);
    let right_is_neighbor = value_right == plot_idx;

    let point_top = Direction::Up.mutate(point);
    let value_top = plots.get(&point_top).unwrap_or(&-1);
    let top_is_neighbor = value_top == plot_idx;

    let point_down = Direction::Down.mutate(point);
    let value_down = plots.get(&point_down).unwrap_or(&-1);
    let down_is_neighbor = value_down == plot_idx;

    let neighbors = [
        &left_is_neighbor,
        &right_is_neighbor,
        &top_is_neighbor,
        &down_is_neighbor,
    ];
    let n_neighbors = neighbors.iter().filter(|it| ***it).count();

    let n_corners = match n_neighbors {
        0 => {
            // no neighbors = 4 corners
            4
        }
        1 => {
            // line edge - 2 corners
            2
        }
        2 => {
            if (left_is_neighbor && right_is_neighbor) || (top_is_neighbor && down_is_neighbor) {
                // line -> 0 corners
                0
            } else {
                // corner - check diagonal
                // links & rechts OF boven & onder ? -> 0
                // else: corner (todo: check diagonal)
                // dbg!(point);
                // draw(plots);
                // todo!()
                1
            }
        }
        3 => {
            // check diagonals
            if !top_is_neighbor {
                todo!()
            } else if !right_is_neighbor {
                todo!()
            } else if !down_is_neighbor {
                // check up left and right
                let diagonal_1 = (point.0 - 1, point.1 - 1);
                let value_diagonal_1 = plots.get(&diagonal_1).unwrap_or(&-1) == plot_idx;
                let diagonal_2 = (point.0 - 1, point.1 + 1);
                let value_diagonal_2 = plots.get(&diagonal_2).unwrap_or(&-1) == plot_idx;

                (value_diagonal_1 as i64) + (value_diagonal_2 as i64)
            } else if !left_is_neighbor {
                todo!()
            } else {
                unreachable!("Huh??");
            }
        }
        4 => {
            // all neighbors = no corners
            0
        }
        _ => {
            unreachable!("Max 4 corners!")
        }
    };

    n_corners
}

fn count_sides(target_plot: i64, plots: &PlotMap) -> i64 {
    // debug function
    let mut result = 0;
    for (point, plot_idx) in plots {
        if &target_plot != plot_idx {
            continue;
        }

        let n_corners = count_corners(point, plot_idx, plots);

        result += n_corners;
    }

    result
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    // uses 'number of sides' instead of 'perimeter'
    let plots = plotmap_from_file(file);
    draw(&plots);

    let mut area_per_plot_idx: HashMap<i64, i64> = Default::default();
    let mut corners_per_plot_idx: HashMap<i64, i64> = Default::default();

    for (point, plot_idx) in &plots {
        *area_per_plot_idx.entry(*plot_idx).or_default() += 1;
        let n_corners = count_corners(point, plot_idx, &plots);

        *corners_per_plot_idx.entry(*plot_idx).or_default() += n_corners;
    }

    dbg!(&corners_per_plot_idx);

    Ok(area_per_plot_idx
        .iter()
        .map(|(region, area)| {
            let sides = corners_per_plot_idx[region];
            sides * area
        })
        .sum())
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

#[tokio::test]
async fn test_counting_corners() {
    let grid = plotmap_from_string(
        r"YXXX
YZZZ
YVVV
VVVV",
    );
    // Y = 0
    // X = 1
    // Z = 2
    // V = 3
    // draw(&grid);

    /// Horizontal (Z/2)

    // top left should have 2 corners
    assert_eq!(count_corners(&(1, 1), &2, &grid), 2);
    // center should have 0 corners
    assert_eq!(count_corners(&(1, 2), &2, &grid), 0);
    // top right should have 2 corners
    assert_eq!(count_corners(&(1, 3), &2, &grid), 2);
    // line should have 4 sides
    assert_eq!(count_sides(2, &grid), 4);

    /// Vertical (Y/0)
    assert_eq!(count_corners(&(0, 0), &0, &grid), 2);
    assert_eq!(count_corners(&(1, 0), &0, &grid), 0);
    assert_eq!(count_corners(&(2, 0), &0, &grid), 2);
    // line should have 4 sides
    assert_eq!(count_sides(0, &grid), 4);

    /// Weird (V/3)
    // bottom left:
    assert_eq!(count_corners(&(3, 0), &3, &grid), 2);
    assert_eq!(count_corners(&(3, 1), &3, &grid), 1);
    assert_eq!(count_sides(3, &grid), 6)
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
