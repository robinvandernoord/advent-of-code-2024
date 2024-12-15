#![allow(dead_code)]

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn repeat_str(chars: &str, times: usize) -> String {
    (0..times).map(|_| chars).collect()
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Instruction {
    Up,
    Right,
    Down,
    Left,
}

impl Instruction {
    fn from_char(char: &char) -> Self {
        match char {
            '^' => Self::Up,
            '>' => Self::Right,
            'v' => Self::Down,
            '<' => Self::Left,
            _ => unreachable!("Unsupported instruction"),
        }
    }

    fn delta(&self) -> Point {
        match self {
            Instruction::Up => (-1, 0),
            Instruction::Right => (0, 1),
            Instruction::Down => (1, 0),
            Instruction::Left => (0, -1),
        }
    }

    fn test(&self, robot: &Point, map: &Map) -> bool {
        // returns whether the move is possible
        // apply 'delta' until empty space or wall:
        let delta = self.delta();

        let mut nxt_point: Point = *robot;

        loop {
            nxt_point = (nxt_point.0 + delta.0, nxt_point.1 + delta.1);
            let nxt_char = map.get(&nxt_point).expect("Out of bounds??");

            match nxt_char.kind {
                MapEntryType::Empty => return true, // yay
                MapEntryType::Wall => return false, // aww
                MapEntryType::Chest => continue,    // do nothing
                MapEntryType::Robot => {
                    panic!("Two robots??")
                }
            }
        }
    }

    fn apply(&self, robot: &Point, map: &mut Map) -> Point {
        // applies the move and returns the new robot location
        let delta = self.delta();

        // #O.OO@ -> #OOO@.
        // #OOO.@ -> #OOO@.

        // 'robot' -> .
        let robot_entry = map
            .insert(*robot, MapEntry::empty())
            .expect("There must be a robot here");

        // 'nxt' -> @
        let mut nxt_point = (robot.0 + delta.0, robot.1 + delta.1);
        let mut nxt_entry = &map
            .insert(nxt_point, robot_entry)
            .expect("Must be something here!");

        let next_robot = nxt_point;

        let mut has_chest = nxt_entry.is_chest();
        // the rest becomes O
        while has_chest {
            nxt_point = (nxt_point.0 + delta.0, nxt_point.1 + delta.1);
            nxt_entry = map.get(&nxt_point).expect("Should be something here");

            match nxt_entry.kind {
                MapEntryType::Robot => {
                    panic!("Two robots??")
                }
                MapEntryType::Chest => {
                    // update position, don't move anything
                    continue;
                }
                MapEntryType::Empty => {
                    // insert O, stop
                    map.insert(nxt_point, MapEntry::chest());
                    has_chest = false;
                }
                MapEntryType::Wall => {
                    has_chest = false;
                }
            }
        }

        next_robot
    }
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum MapEntryType {
    Wall,
    Chest, // Box is already a rust keyword
    Robot,
    Empty,
}

#[derive(Debug, Hash, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
struct MapEntry {
    kind: MapEntryType,
}

impl MapEntry {
    fn from_char(char: &char) -> Self {
        let kind = match char {
            '#' => MapEntryType::Wall,
            'O' => MapEntryType::Chest,
            '@' => MapEntryType::Robot,
            '.' => MapEntryType::Empty,
            other => {
                dbg!(other);
                unreachable!("Unsupported character?")
            }
        };

        Self { kind }
    }

    fn empty() -> Self {
        Self {
            kind: MapEntryType::Empty,
        }
    }

    fn robot() -> Self {
        Self {
            kind: MapEntryType::Robot,
        }
    }

    fn chest() -> Self {
        Self {
            kind: MapEntryType::Chest,
        }
    }

    fn as_char(&self) -> char {
        match self.kind {
            MapEntryType::Wall => '#',
            MapEntryType::Chest => 'O',
            MapEntryType::Robot => '@',
            MapEntryType::Empty => '.',
        }
    }

    fn is(&self, other: MapEntryType) -> bool {
        self.kind == other
    }

    fn is_robot(&self) -> bool {
        self.is(MapEntryType::Robot)
    }

    fn is_empty(&self) -> bool {
        self.is(MapEntryType::Empty)
    }

    fn is_chest(&self) -> bool {
        self.is(MapEntryType::Chest)
    }

    fn is_wall(&self) -> bool {
        self.is(MapEntryType::Wall)
    }
}

// impl Into<char> for MapEntry {
//     fn into(self) -> char {
//         self.to_char()
//     }
// }
//
// impl Into<char> for &MapEntry {
//     fn into(self) -> char {
//         self.to_char()
//     }
// }

type Point = (i64, i64);
type Map = BTreeMap<Point, MapEntry>;

fn draw(map: &Map) {
    let mut y = 0;
    for (point, entry) in map {
        if point.0 != y {
            println!();
            y = point.0;
        }

        let chr = entry.as_char();
        print!("{chr}");
    }

    println!();
}

fn draw_fancy(map: &Map) {
    use std::thread::sleep;
    use std::time::Duration;

    sleep(Duration::from_millis(100));

    let mut y = 0;
    for (point, entry) in map {
        if point.0 != y {
            println!();
            y = point.0;
        }

        let chr = entry.as_char();
        print!("{chr}");
    }

    print!("{}", repeat_str("\n", 10));
}

fn gps(map: &Map) -> i64 {
    let mut result = 0;
    for (point, entry) in map {
        if entry.is_chest() {
            result += point.0 * 100 + point.1;
        }
    }

    result
}

fn count_chests(map: &Map) -> i64 {
    map.iter().filter(|(_, entry)| entry.is_chest()).count() as i64
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let mut instructions_mode = false;
    let mut map: Map = Default::default();
    let mut instructions: Vec<Instruction> = Default::default();
    let mut robot: Point = (0, 0);

    for (y, line) in file.map_while(Result::ok).enumerate() {
        if line.is_empty() {
            instructions_mode = true;
        } else if instructions_mode {
            for char in line.chars() {
                instructions.push(Instruction::from_char(&char))
            }
        } else {
            // map mode
            for (x, char) in line.chars().enumerate() {
                let point = (y as i64, x as i64);
                let entry = MapEntry::from_char(&char);
                map.insert(point, entry);
                if entry.is_robot() {
                    robot = point;
                }
            }
        }
    }

    let initial_chest_count = count_chests(&map);

    for instruction in &instructions {
        if instruction.test(&robot, &map) {
            robot = instruction.apply(&robot, &mut map);
        }

        // draw_fancy(&map);

        assert_eq!(
            count_chests(&map),
            initial_chest_count,
            "Amount of boxes changed!"
        );
    }

    draw(&map);
    Ok(gps(&map))
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
async fn test_simple_minimal_1() {
    let answer = 2028;

    let file = read_lines("minimal1.txt")
        .await
        .expect("Should be able to read minimal1.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple_minimal_2() {
    let answer = 10092;

    let file = read_lines("minimal2.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 1526018;

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
