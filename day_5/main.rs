#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Nodes = HashMap<i64, HashSet<i64>>;

fn task_valid(task: &Vec<i64>, nodes: &Nodes) -> bool {
    let empty: HashSet<i64> = HashSet::new();

    let mut nodes_inv: Nodes = Nodes::new();

    // build inverse relationships:
    for step in task {
        for dep in nodes.get(step).unwrap_or(&empty) {
            nodes_inv.entry(*dep).or_default().insert(*step);
        }
    }

    let mut seen: HashSet<i64> = HashSet::new();

    for step in task {
        let depends = nodes_inv.get(step).unwrap_or(&empty);

        if depends != &seen {
            // error in dependency tree
            return false;
        }
        seen.insert(*step);
    }

    true
}

fn try_task(task: &Vec<i64>, nodes: &Nodes) -> i64 {
    if task_valid(task, nodes) {
        // middle value
        task[task.len() / 2]
    } else {
        0
    }
}

fn parse_pages(file: FileHandle) -> (Vec<Vec<i64>>, Nodes) {
    let empty_string: &'static str = "";
    let mut nodes: Nodes = HashMap::new();
    let mut tasks = vec![];
    let mut first_half = true;

    for line in file.map_while(Result::ok) {
        if line == empty_string {
            first_half = false;
        } else if first_half {
            let mut parts = line.split("|");
            let left = parts
                .next()
                .expect("Should be 2")
                .parse::<i64>()
                .expect("Should be a number");

            let right = parts
                .next()
                .expect("Should be 2")
                .parse::<i64>()
                .expect("Should be a number");

            nodes.entry(left).or_default().insert(right);
        } else {
            let parts = line.split(",");
            let task: Vec<i64> = parts
                .map(|it| it.parse().expect("Should be number"))
                .collect();
            tasks.push(task)
        }
    }

    (tasks, nodes)
}

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let (tasks, nodes) = parse_pages(file);

    let mut result = 0;

    for task in tasks {
        result += try_task(&task, &nodes);
    }

    Ok(result)
}

fn rebuild_nodes_relevant_inv(task: &Vec<i64>, nodes: &Nodes) -> Nodes {
    let mut nodes_relevant_inv: Nodes = Nodes::new();
    let empty = HashSet::new();

    // build inverse relationships:
    for step in task {
        for dep in nodes.get(step).unwrap_or(&empty) {
            if task.contains(dep) {
                nodes_relevant_inv.entry(*dep).or_default().insert(*step);
            }
        }
    }

    nodes_relevant_inv
}

fn fix_task(task: &[i64], nodes: &Nodes) -> i64 {
    let mut todo = task.to_owned();

    let max_loop = 1000;
    let mut curr_loop = 0;

    let mut result: Vec<i64> = vec![];

    loop {
        if curr_loop > max_loop {
            panic!("Looped too much!!!")
        }
        // rip perf but ok:
        let nodes_relevant_inv = rebuild_nodes_relevant_inv(&todo, nodes);

        for (idx, item) in todo.iter().enumerate() {
            if !nodes_relevant_inv.contains_key(item) {
                // no dependency yippie
                result.push(*item);
                todo.remove(idx);

                break;
            }
        }

        if todo.is_empty() {
            break;
        }

        curr_loop += 1;
    }

    result[result.len() / 2]
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    let (tasks, nodes) = parse_pages(file);

    let mut result = 0;

    for task in tasks {
        if !task_valid(&task, &nodes) {
            result += fix_task(&task, &nodes);
        }
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
    let answer = 143;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 7365;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read simple.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced_minimal() {
    let answer = 123;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_advanced() {
    let answer = 5770;

    let file = read_lines("simple.txt")
        .await
        .expect("Should be able to read advanced.txt");

    assert_eq!(advanced(file).await.expect("Oof 2"), answer);
}
