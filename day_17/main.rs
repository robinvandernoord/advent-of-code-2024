#![allow(dead_code)]

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Registers = HashMap<char, i64>;

fn operand_to_combo(operand: i64, register: &Registers) -> i64 {
    match operand {
        4 => register[&'A'],
        5 => register[&'B'],
        6 => register[&'C'],
        // 7 => {
        //     unreachable!("Combo operand 7 is reserved and will not appear in valid programs.")
        // }
        _ => operand,
    }
}

fn run(
    ptr: usize,
    instruction: i64,
    operand: i64,
    registers: &mut Registers,
    result: &mut Vec<i64>,
) -> usize {
    let combo = operand_to_combo(operand, registers);

    match instruction {
        0 => {
            // adv
            let numerator = registers[&'A'];
            let denominator = 2_i64.pow(combo as u32);

            let result = numerator / denominator;
            registers.insert('A', result);
            ptr + 2
        }
        1 => {
            // bxl
            registers.insert('B', registers[&'B'] ^ operand);
            ptr + 2
        }
        2 => {
            // bst
            registers.insert('B', combo % 8);
            ptr + 2
        }
        3 => {
            // jnz
            if registers[&'A'] != 0 {
                // jump to literal operand:
                operand as usize
            } else {
                ptr + 2
            }
        }
        4 => {
            // bxc
            registers.insert('B', registers[&'B'] ^ registers[&'C']);
            ptr + 2
        }
        5 => {
            // out
            result.push(combo % 8);
            ptr + 2
        }
        6 => {
            // bdv
            let numerator = registers[&'A'];
            let denominator = 2_i64.pow(combo as u32);

            let result = numerator / denominator;
            registers.insert('B', result);
            ptr + 2
        }
        7 => {
            // cdv
            let numerator = registers[&'A'];
            let denominator = 2_i64.pow(combo as u32);

            let result = numerator / denominator;
            registers.insert('C', result);
            ptr + 2
        }
        _ => {
            unreachable!("This is a 3 bit computer.")
        }
    }
}

fn process(instructions: &[i64], registers: &mut Registers) -> String {
    let mut ptr = 0;
    let mut results: Vec<i64> = Default::default();

    while ptr < instructions.len() {
        let instruction = instructions[ptr];
        let operand = instructions[ptr + 1];

        ptr = run(ptr, instruction, operand, registers, &mut results)
    }

    let results: Vec<_> = results.iter().map(|it| it.to_string()).collect();
    results.join(",")
}

async fn simple(file: FileHandle) -> anyhow::Result<String> {
    let mut registers: Registers = Default::default();
    let mut instruction: Vec<i64> = Default::default();

    for line in file.map_while(Result::ok) {
        if line.is_empty() {
            continue;
        } else if line.starts_with("Register ") {
            let line = line.strip_prefix("Register ").expect("?");
            let mut parts = line.split(":");
            let register = parts
                .next()
                .expect("Should have 2 parts")
                .chars()
                .next()
                .expect("Should have at least 1 char");
            let value: i64 = parts
                .next()
                .expect("Should have 2 parts")
                .trim()
                .parse()
                .expect("Should be valid int");

            registers.insert(register, value);
        } else {
            // program
            let mut parts = line.split(":");
            parts.next();
            let instructions: Vec<i64> = parts
                .next()
                .expect("Should have 2 parts")
                .trim()
                .split(",")
                .map(|it| it.parse().expect("Should be valid int"))
                .collect();

            instruction.extend(instructions);
        }
    }

    Ok(process(&instruction, &mut registers))
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

fn empty_registers() -> Registers {
    let mut registers: Registers = Default::default();
    registers.insert('A', 0);
    registers.insert('B', 0);
    registers.insert('C', 0);

    registers
}

#[tokio::test]
async fn test_instructions() {
    // If register C contains 9, the program 2,6 would set register B to 1.
    let mut registers = empty_registers();
    registers.insert('C', 9);
    assert_eq!(process(&[2, 6], &mut registers), String::new());
    assert_eq!(registers[&'B'], 1);

    // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
    let mut registers = empty_registers();
    registers.insert('A', 10);
    assert_eq!(
        process(&[5, 0, 5, 1, 5, 4], &mut registers),
        "0,1,2".to_string()
    );

    // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
    let mut registers = empty_registers();
    registers.insert('A', 2024);
    assert_eq!(
        process(&[0, 1, 5, 4, 3, 0], &mut registers),
        "4,2,5,6,7,7,7,7,3,1,0".to_string()
    );
    assert_eq!(registers[&'A'], 0);

    // If register B contains 29, the program 1,7 would set register B to 26.
    let mut registers = empty_registers();
    registers.insert('B', 29);
    assert_eq!(process(&[1, 7], &mut registers), String::new());
    assert_eq!(registers[&'B'], 26);

    // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
    let mut registers = empty_registers();
    registers.insert('B', 2024);
    registers.insert('C', 43690);
    assert_eq!(process(&[4, 0], &mut registers), String::new());
    assert_eq!(registers[&'B'], 44354);
}

#[tokio::test]
async fn test_simple_minimal() {
    let answer = "4,6,3,5,6,3,5,2,1,0";

    let file = read_lines("minimal.txt")
        .await
        .expect("Should be able to read minimal.txt");

    assert_eq!(simple(file).await.expect("Oof 1"), answer);
}

#[tokio::test]
async fn test_simple() {
    let answer = "3,6,3,7,0,7,0,3,0";

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
