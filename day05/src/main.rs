use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

fn read<R: Read>(io: R) -> Result<Vec<String>, Error> {
    let br = BufReader::new(io);
    br.lines().collect()
}

fn calculator(pos: &str, range: (u8, u8)) -> u8{
    let half = (range.1 - range.0) / 2;
    match pos.chars().nth(0) {
        Some(x) if x == 'F' || x == 'L' => calculator(&pos[1..], (range.0, range.0 + half)),
        Some (_) => calculator(&pos[1..], (range.0 + half + 1, range.1)),
        None => range.0
    }
}

fn main() -> Result<(), Error> {
    let lines = read(File::open("input")?)?;
    let mut max: u32 = 0;
    let range = (0, 127);
    let mut results: Vec<u32> = Vec::new();
    for line in lines {
        let row = calculator(&line[..7], range);
        let col = calculator(&line[7..], (0, 7));
        let res: u32 = row as u32 * 8 + col as u32;
        if res > max {
            max = res;
        }
        results.push(res);
    } 
    results.sort();
    for elem in (49..873).into_iter() {
        match &results[..].into_iter().find(|x| **x == elem) {
            Some(_) => (),
            None => println!("Missing seat: {}", elem)
        }
    }
    println!("Max: {}", max);
    Ok(())
}
