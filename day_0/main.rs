#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

async fn simple(file: FileHandle) -> anyhow::Result<i64> {
    for line in file.flatten() {
        println!("{}", line);
    }
    Ok(0)
}

async fn advanced(file: FileHandle) -> anyhow::Result<i64> {
    for line in file.flatten() {
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
    let answer = 0;

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}
// 
// #[tokio::test]
// async fn test_simple() {
//     let answer = 0;
// 
//     let file = read_lines("simple.txt")
//         .await
//         .expect("Should be able to read simple.txt");
// 
//     assert_eq!(simple(file).await.expect("Oof 1"), answer);
// }
// 
// #[tokio::test]
// async fn test_advanced_minimal() {
//     let answer = 0;
// 
//     let file = read_lines("minimal.txt")
//         .await
//         .expect("Should be able to read minimal.txt");
// 
//     assert_eq!(simple(file).await.expect("Oof 1"), answer);
// }
// 
// #[tokio::test]
// async fn test_advanced() {
//     let answer = 0;
// 
//     let file = read_lines("advanced.txt")
//         .await
//         .expect("Should be able to read advanced.txt");
// 
//     assert_eq!(advanced(file).await.expect("Oof 2"), answer);
// }
