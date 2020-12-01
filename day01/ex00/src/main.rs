use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn check_vector(value: &u64, vec: &Vec<u64>) -> Option<u64>{
    for i in vec{
        if value + i == 2020 {
            return Some(value * i);
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input")?)?;
    for i in &vec {
        match check_vector(i, &vec){
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
