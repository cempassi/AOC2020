use regex::Regex;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

#[derive(Debug)]
struct Passwd {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl Passwd {
    fn validate_first(self: &Self) -> bool {
        let count: usize = self.password.matches(self.letter).count();
        if count >= self.min && count <= self.max {
            true
        } else {
            false
        }
    }

    fn validate_second(self: &Self) -> bool {
        let first: char = match self.password.chars().nth(self.min - 1) {
            Some(x) => x,
            None => '@'
        };
        let second: char = match self.password.chars().nth(self.max - 1) {
            Some(x) => x,
            None => '@'
        };
        if first == self.letter && second == self.letter {
                false
        }
        else if first == self.letter || second == self.letter {
                true
        }else {
            false
        }
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Passwd>, Error> {
    let br = BufReader::new(io);
    let re = Regex::new(r"(\d*)-(\d*)\s([a-zA-Z]):\s([a-zA-Z]*)").unwrap();
    br.lines()
        .map(|line| {
            line.and_then(|s| {
                let captures = re.captures(s.as_str()).unwrap();
                Ok(Passwd {
                    min: captures.get(1).unwrap().as_str().parse::<usize>().unwrap(),
                    max: captures.get(2).unwrap().as_str().parse::<usize>().unwrap(),
                    letter: captures.get(3).unwrap().as_str().chars().next().unwrap(),
                    password: captures.get(4).unwrap().as_str().to_owned(),
                })
            })
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let passwords = read(File::open("input")?)?;
    let valid_first = passwords[..]
        .into_iter()
        .filter(|x| x.validate_first())
        .count();
    println!("Result: {:?}", valid_first);
    let valid_second = passwords
        .into_iter()
        .filter(|x| x.validate_second())
        .count();
    println!("Result: {:?}", valid_second);
    Ok(())
}
