#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Relationships = HashMap<String, HashSet<String>>;

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    let empty_hashset: HashSet<String> = HashSet::new();

    let mut relationships = Relationships::new();

    for line in file.map_while(Result::ok) {
        let mut parts = line.split("-");
        let first_node_name = parts.next().expect("Should have 2 parts");
        let second_node_name = parts.next().expect("Should have 2 parts");

        relationships
            .entry(first_node_name.to_string())
            .or_default()
            .insert(second_node_name.to_string());
        relationships
            .entry(second_node_name.to_string())
            .or_default()
            .insert(first_node_name.to_string());
    }

    let mut triples = HashSet::new();

    for (source, target) in &relationships {
        if !source.starts_with("t") {
            continue;
        }

        for candidate in target {
            let candidate_relationships = relationships.get(candidate).unwrap_or(&empty_hashset);

            for subcandidate in candidate_relationships {
                if target.contains(subcandidate) {
                    let mut unsorted = [source, candidate, subcandidate];
                    unsorted.sort();
                    triples.insert((unsorted[0], unsorted[1], unsorted[2]));
                }
            }
        }
    }

    // let mut triples: Vec<_> = triples.into_iter().collect();
    // triples.sort();
    //
    // dbg!(triples);

    Ok(triples.len() as i64)
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
    let answer = 7;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = 1327;

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
