use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

fn read<R: Read>(io: R) -> Result<Vec<Vec<String>>, Error> {
    let br = BufReader::new(io);
    let mut total: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    for line in br.lines() {
        if let Ok(l) = line {
            match l.is_empty() {
                true => {
                    total.push(group.clone());
                    group.clear();
                }
                false => {
                    group.push(l);
                }
            }
        }
    }
    total.push(group.clone());
    return Ok(total);
}

fn second_part(readed: &Vec<Vec<String>>) -> usize {
    let mut results: Vec<HashSet<char>> = Vec::new();

    for mut group in readed.clone() {
        let mut set: HashSet<char> = HashSet::new();
        if let Some(x) = group.pop() {
            x.chars().for_each(|c| match set.insert(c) {
                true => (),
                false => (),
            });
            if group.is_empty() {
                results.push(set.clone());
            } else {
                group.into_iter().for_each(|line| {
                    let interset: HashSet<char> = line.chars().collect();
                    let mut holder: HashSet<char> = HashSet::new();
                    for x in set.intersection(&interset) {
                        holder.insert(*x);
                    }
                    set = holder;
                });
                results.push(set.clone());
            }
        }
    }

    results.into_iter().fold(0, |acc, x| acc + x.len())
}

fn first_part(readed: &Vec<Vec<String>>) -> usize {
    let mut results: Vec<HashSet<char>> = Vec::new();

    for group in readed {
        let mut set: HashSet<char> = HashSet::new();
        group.into_iter().for_each(|line| {
            line.chars().for_each(|c| match set.insert(c) {
                true => (),
                false => (),
            })
        });
        results.push(set.clone());
    }
    results.into_iter().fold(0, |acc, x| acc + x.len())
}

fn main() -> Result<(), Error> {
    let readed: Vec<Vec<String>> = read(File::open("input")?)?;

    println!(
        "Result first part: {}, second: {}",
        first_part(&readed),
        second_part(&readed)
    );
    Ok(())
}
