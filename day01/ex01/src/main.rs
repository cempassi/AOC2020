use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error, ErrorKind};

fn read<R: Read>(io: R) -> Result<Vec<u64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn check_vector(first: &u64, second: &u64, vec: &[u64]) -> Option<u64> {
    for i in vec {
        if first + second + i == 2020 {
            return Some(first * second * i);
        }
    }
    None
}

fn main() -> Result<(), Error> {
    let vec = read(File::open("input")?)?;

    for (index, first) in (0..vec.len()).zip(vec[..].into_iter()) {
        let slice1 = &vec[index..];
        let slice2 = &slice1[1..];

        for second in slice1 {
            match check_vector(first, second, slice2) {
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
