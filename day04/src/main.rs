use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

fn check(mapped: &HashMap<&str, String>) -> bool {
    if mapped.contains_key("pid") == false {
        false
    }
    else if mapped.contains_key("iyr") == false{
        false
    }
    else if mapped.contains_key("eyr") == false{
        false
    }
    else if mapped.contains_key("byr") == false{
        false
    }
    else if mapped.contains_key("hgt") == false{
        false
    }
    else if mapped.contains_key("hcl") == false{
        false
    }
    else if mapped.contains_key("ecl") == false{
        false
    }
    else {
        true
    }
}

fn try_new(strings: Vec<String>, regset: &Vec<Regex>) -> bool {
    let mut mapped = HashMap::new();
    for string in strings {
        regset[..].into_iter().for_each(|re| {
            re.captures(string.as_str()).and_then(|c| {
                if let Some(x) = c.get(1) {
                    match x.as_str() {
                        "pid" => mapped.insert("pid", c.get(2).unwrap().as_str().to_string()),
                        "cid" => mapped.insert("cid", c.get(2).unwrap().as_str().to_string()),
                        "iyr" => mapped.insert("iyr", c.get(2).unwrap().as_str().to_string()),
                        "eyr" => mapped.insert("eyr", c.get(2).unwrap().as_str().to_string()),
                        "byr" => mapped.insert("byr", c.get(2).unwrap().as_str().to_string()),
                        "hgt" => mapped.insert("hgt", c.get(2).unwrap().as_str().to_string()),
                        "hcl" => mapped.insert("hcl", c.get(2).unwrap().as_str().to_string()),
                        "ecl" => mapped.insert("ecl", c.get(2).unwrap().as_str().to_string()),
                        _ => None,
                    };
                    Some(())
                } else {
                    None
                }
            });
        })
    }
    if mapped.len() == 8 && check(&mapped){
        return true;
    }
    if mapped.len() == 7 && check(&mapped){
        return true;
    }
    return false;
}

fn read<R: Read>(io: R) -> Result<Vec<Vec<String>>, Error> {
    let br = BufReader::new(io);
    let mut passports: Vec<Vec<String>> = vec![vec![]];
    let mut passport: Vec<String> = vec![];
    for line in br.lines() {
        if let Ok(l) = line {
            match l.is_empty() {
                true => {
                    passports.push(passport.clone());
                    passport.clear();
                }
                false => passport.push(l),
            }
        }
    }
    passports.push(passport.clone());
    return Ok(passports);
}

fn main() -> Result<(), Error> {
    let readed: Vec<Vec<String>> = read(File::open("input")?)?;
    let mut regexes: Vec<Regex> = vec![];
    let mut passports:u32 = 0;
    regexes.push(Regex::new(r"(byr):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(pid):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(cid):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(iyr):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(eyr):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(hgt):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(hcl):([\w#]+)").unwrap());
    regexes.push(Regex::new(r"(ecl):([\w#]+)").unwrap());
    for passport in readed {
        match try_new(passport, &regexes) {
            true => passports += 1,
            false => (),
        }
    }
    println!("Resut: {}", passports);
    Ok(())
}
