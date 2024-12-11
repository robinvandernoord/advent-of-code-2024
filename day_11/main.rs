#![allow(dead_code)]
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::{self, JoinHandle};

fn even_digits(num: &i64) -> bool {
    let string = num.to_string();
    string.len() % 2 == 0
}

fn split_in_half(num_even_digits: &i64) -> (i64, i64) {
    let string = num_even_digits.to_string();
    let (str_1, str_2) = string.split_at(string.len() / 2);

    let num_1 = str_1.parse().expect("Should still be number");
    let num_2 = str_2.parse().expect("Should still be number");
    (num_1, num_2)
}

fn blink(stones: &mut Vec<i64>, times: i64) {
    if times == 0 {
        return;
    }
    // eprintln!("{} -> {}", times, stones.len());

    for (idx, stone) in stones.clone().iter().enumerate() {
        if stone == &0 {
            stones[idx] = 1
        } else if even_digits(stone) {
            // note: order is ignored for performance here; could be required later?
            let (first, second) = split_in_half(stone);
            stones[idx] = first;
            stones.push(second);
        } else {
            stones[idx] = stone * 2024;
        }
    }

    blink(stones, times - 1)
}

fn blink_v2(stone: i64, times: i64) -> i64 {
    // similar logic to v1 but goes in chunks to reduce memory usage
    // (which became > 50% on part 2!)

    if times == 0 {
        return 1;
    }

    if stone == 0 {
        blink_v2(1, times - 1)
    } else if even_digits(&stone) {
        // note: order is ignored for performance here; could be required later?
        let (first, second) = split_in_half(&stone);
        
        blink_v2(first, times - 1) + blink_v2(second, times - 1)
    } else {
        blink_v2(stone * 2024, times - 1)
    }
}

async fn simple(file: FileHandle, n: i64) -> anyhow::Result<i64> {
    if let Some(line) = file.map_while(Result::ok).next() {
        let mut stones: Vec<i64> = line
            .split(' ')
            .map(|it| it.parse().expect("Should be numbers"))
            .collect();

        blink(&mut stones, n);
        return Ok(stones.len() as i64);
    }
    Ok(0)
}

fn join_all<T: Sized>(futures: Vec<JoinHandle<T>>) -> Vec<T> {
    futures
        .into_iter()
        .map(|it| it.join().expect("Should be fine"))
        .collect()
}

async fn advanced(file: FileHandle, n: i64) -> anyhow::Result<i64> {
    if let Some(line) = file.map_while(Result::ok).next() {
        let stones: Vec<i64> = line
            .split(' ')
            .map(|it| it.parse().expect("Should be numbers"))
            .collect();

        let mut futures = vec![];
        for stone in stones {
            // one thread per initial stone
            futures.push(thread::spawn(move || blink_v2(stone, n)));
        }

        return Ok(join_all(futures).iter().sum());
    }
    Ok(0)
}

// -- tests --

type FileHandle = io::Lines<io::BufReader<File>>;

async fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<FileHandle> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// #[tokio::test]
// async fn test_simple_minimal_6() {
//     let file = read_lines("minimal.txt")
//         .await
//         .expect("Should be able to read minimal.txt");
//
//     assert_eq!(simple(file, 6).await.expect("Oof 1"), 22);
// }
//
// #[tokio::test]
// async fn test_simple_minimal_25() {
//     let file = read_lines("minimal.txt")
//         .await
//         .expect("Should be able to read minimal.txt");
//
//     assert_eq!(simple(file, 25).await.expect("Oof 1"), 55312);
// }
//
// #[tokio::test]
// async fn test_simple() {
//     let answer = 203457;
//
//     let file = read_lines("input.txt")
//         .await
//         .expect("Should be able to read input.txt");
//
//     assert_eq!(simple(file, 25).await.expect("Oof 1"), answer);
// }

#[tokio::test]
async fn test_simple_minimal_6_v2() {
    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file, 6).await.expect("Oof 1"), 22);
}

#[tokio::test]
async fn test_simple_minimal_25_v2() {
    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(advanced(file, 25).await.expect("Oof 1"), 55312);
}

#[tokio::test]
async fn test_simple_v2() {
    let answer = 203457;

    let file = read_lines("input.txt")
        .await
        .expect("Should be able to read input.txt");

    assert_eq!(advanced(file, 25).await.expect("Oof 1"), answer);
}

// #[tokio::test]
// async fn test_simulate_blink_v1() {
//     /**
//     50 -> 1
//     49 -> 1
//     48 -> 1
//     47 -> 2
//     46 -> 4
//     45 -> 4
//     44 -> 7
//     43 -> 14
//     42 -> 16
//     41 -> 20
//     40 -> 39
//     39 -> 62
//     38 -> 81
//     37 -> 110
//     36 -> 200
//     35 -> 328
//     34 -> 418
//     33 -> 667
//     32 -> 1059
//     31 -> 1546
//     30 -> 2377
//     29 -> 3572
//     28 -> 5602
//     27 -> 8268
//     26 -> 12343
//     25 -> 19778
//     24 -> 29165
//     23 -> 43726
//     22 -> 67724
//     21 -> 102131
//     20 -> 156451
//     19 -> 234511
//     18 -> 357632
//     17 -> 549949
//     16 -> 819967
//     15 -> 1258125
//     14 -> 1916299
//     13 -> 2886408
//     12 -> 4414216
//     11 -> 6669768
//     10 -> 10174278
//     9 -> 15458147
//     8 -> 23333796
//     7 -> 35712308
//     6 -> 54046805
//     5 -> 81997335
//     4 -> 125001266
//     3 -> 189148778
//     2 -> 288114305
//     1 -> 437102505
//     **/
//     let mut v = vec![0];
//     blink(&mut v, 49); // 35.06s
//     assert_eq!(v.len(), 437102505);
// }

// #[tokio::test]
// async fn test_simulate_blink_v2() {
//     let v = vec![0];
//     assert_eq!(blink_v2(&v, 49), 437102505); // 34s
// }

// #[tokio::test]
// async fn test_advanced() {
//     let answer = 0;

//     let file = read_lines("input.txt")
//         .await
//         .expect("Should be able to read input.txt");

//     assert_eq!(advanced(file, 75).await.expect("Oof 2"), answer);
// }
