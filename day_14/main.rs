#![allow(dead_code)]

use once_cell::sync::Lazy;
use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Point = (i64, i64);

fn parse_point(value: &str) -> Option<Point> {
    let mut parts = value.split(",");
    let left = parts.next()?.trim();
    let right = parts.next()?.trim();

    let left_i: i64 = left.parse().ok()?;
    let right_i: i64 = right.parse().ok()?;

    Some((left_i, right_i))
}

fn wrap(number: i64, max: i64) -> i64 {
    // works like Python's modulo:
    number.rem_euclid(max)
}

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct Robot {
    pub position: Point,
    pub velocity: Point,
}

impl Robot {
    fn parse(line: &str) -> Self {
        static RE: Lazy<Regex> =
            Lazy::new(|| Regex::new(r"p=(-?\d+,-?\d+) v=(-?\d+,-?\d+)").unwrap());

        let caps = RE.captures(line).expect("Invalid Robot");
        let mut caps_i = caps.iter();
        caps_i.next();
        let p = caps_i.next().flatten().expect("Should exist").as_str();
        let v = caps_i.next().flatten().expect("Should exist").as_str();

        Self {
            position: parse_point(p).expect("Should be valid position"),
            velocity: parse_point(v).expect("Should be valid velocity"),
        }
    }

    fn moves(&mut self, times: i64, width: i64, height: i64) {
        let new_x = wrap(self.position.0 + (self.velocity.0 * times), width);
        let new_y = wrap(self.position.1 + (self.velocity.1 * times), height);

        assert!(new_y < height);
        assert!(new_y >= 0);

        assert!(new_x < width);
        assert!(new_x >= 0);

        self.position = (new_x, new_y)
    }
}

fn draw(robots: &[Robot], width: i64, height: i64) {
    let mut robots_per_point: HashMap<Point, Vec<&Robot>> = Default::default();

    robots.iter().for_each(|robot| {
        robots_per_point
            .entry(robot.position)
            .or_default()
            .push(robot);
    });

    assert_ne!(robots_per_point.len(), 0, "No robots to print?");

    for y in 0..height {
        for x in 0..width {
            let point: Point = (x, y);
            if let Some(robots) = robots_per_point.remove(&point) {
                print!("{}", robots.len());
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();

    // all should have been printed, otherwise something is funky:
    assert_eq!(robots_per_point.len(), 0, "Unprinted robots left?");
}

async fn simple(file: FileHandle, width: i64, height: i64, times: i64) -> anyhow::Result<i64> {
    let mut robots: Vec<Robot> = Default::default();

    for line in file.map_while(Result::ok) {
        if line.starts_with("#") {
            continue;
        }

        let robot = Robot::parse(&line);
        robots.push(robot);
    }

    // draw(&robots, width, height);
    robots
        .iter_mut()
        .for_each(|robot| robot.moves(times, width, height));
    // draw(&robots, width, height);

    let mut quadrants: [i64; 4] = [0, 0, 0, 0];

    let max_x = width / 2;
    let max_y = height / 2;

    robots.iter().for_each(|robot| {
        // quadrant 1 = 0 .. (width / 2); 0 .. (height / 2)
        // quadrant 2 = (width / 2) .. width; 0 .. (height / 2)
        // quadrant 3 = 0 .. (width / 2); (height / 2) .. height
        // quadrant 4 = (width / 2) .. width; (height / 2) .. height
        use Ordering::*;

        let maybe_quadrant_number =
            match (max_x.cmp(&robot.position.0), max_y.cmp(&robot.position.1)) {
                (Equal, _) | (_, Equal) => None,
                (Less, Less) => Some(0),
                (Greater, Less) => Some(1),
                (Less, Greater) => Some(2),
                (Greater, Greater) => Some(3),
            };

        if let Some(quadrant_number) = maybe_quadrant_number {
            quadrants[quadrant_number] += 1;
        }
    });

    Ok(quadrants
        .into_iter()
        .reduce(|a, b| a * b)
        .expect("What could go wrong"))
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
async fn test_simple_minimal() {
    let answer = 12;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file, 11, 7, 100).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 232253028;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(simple(file, 101, 103, 100).await.expect("Oof 1"), answer);
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
