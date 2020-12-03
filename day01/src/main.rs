use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn check_first(value: &u64, vec: &Vec<u64>) -> Option<u64> {
    for i in vec {
        if value + i == 2020 {
            return Some(value * i);
        }
    }
    None
}

fn first_solution(vec: &Vec<u64>) -> Result<(), Error> {
    for i in vec {
        match check_first(i, &vec) {
            Some(x) => {
                println!("Result: {}", x);
                return Ok(());
            }
            None => (),
        };
    }
    println!("Nothing found");
    Ok(())
}

fn check_second(first: &u64, second: &u64, vec: &[u64]) -> Option<u64> {
    for i in vec {
        if first + second + i == 2020 {
            return Some(first * second * i);
        }
    }
    None
}

fn second_solution(vec: Vec<u64>) -> Result<(), Error> {
    for (index, first) in (0..vec.len()).zip(vec[..].into_iter()) {
        let slice1 = &vec[index..];
        let slice2 = &slice1[1..];

        for second in slice1 {
            match check_second(first, second, slice2) {
                Some(x) => {
                    println!("Result: {}", x);
                    return Ok(());
                }
                None => (),
            };
        }
    }
    println!("Nothing found");
    Ok(())
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input")?)?;
    first_solution(&vec)?;
    second_solution(vec)?;
    Ok(())
}
