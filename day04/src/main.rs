use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

fn check_byr(mapped:&HashMap<&str, String>) -> bool{
    if let Some(x) = mapped.get("byr") {
        if let Ok(birthday) = x.parse::<u32>(){
            if birthday >= 1920 && birthday <= 2002 {
                return true;
            }
        }
        false
    }
    else {false}
}

fn check_iyr(mapped:&HashMap<&str, String>) -> bool{
    if let Some(x) = mapped.get("iyr") {
        if let Ok(issued) = x.parse::<u32>(){
            if issued >= 2010 && issued <= 2020 {
                return true;
            }
        }
        false
    }
    else { false }
}

fn check_eyr(mapped:&HashMap<&str, String>) -> bool{
    if let Some(x) =  mapped.get("eyr") {
        if let Ok(expire) = x.parse::<u32>(){
            if expire >= 2020 && expire <= 2030 {
                return true;
            }
        }
    }
    false
}

fn check_hgt(mapped:&HashMap<&str, String>) -> bool{
    let key = mapped.get("uni").unwrap();
    let value = mapped.get("hgt").unwrap().parse::<u32>().unwrap();
    if key == "cm" && value >= 150 && value <= 193 {
        return true;
    }
    else if key == "in" && value >= 59 && value <= 76{
        return true;
    }
    return false;
}

fn check(mapped: &HashMap<&str, String>) -> bool {
    if mapped.contains_key("byr") == false || check_byr(mapped) == false {
        false
    }
     else if mapped.contains_key("iyr") == false || check_iyr(mapped) == false {
         false
     }
     else if mapped.contains_key("eyr") == false || check_eyr(mapped) == false {
         false
     }
     else if mapped.contains_key("hgt") == false || check_hgt(mapped) == false {
         false
     }
     else if mapped.contains_key("pid") == false {
         false
     }
     else if mapped.contains_key("hcl") == false {
         false
     }
     else if mapped.contains_key("ecl") == false {
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
                        "byr" => mapped.insert("byr", c.get(2).unwrap().as_str().to_string()),
                        "iyr" => mapped.insert("iyr", c.get(2).unwrap().as_str().to_string()),
                        "pid" => mapped.insert("pid", c.get(2).unwrap().as_str().to_string()),
                        "eyr" => mapped.insert("eyr", c.get(2).unwrap().as_str().to_string()),
                        "hgt" => {
                            if let Some(_) = c.get(0).unwrap().as_str().find("cm"){
                                mapped.insert("uni", c.get(6).unwrap().as_str().to_string());
                                mapped.insert("hgt", c.get(5).unwrap().as_str().to_string())
                            }
                            else if let Some(_) = c.get(0).unwrap().as_str().find("in"){
                                mapped.insert("uni", c.get(4).unwrap().as_str().to_string());
                                mapped.insert("hgt", c.get(3).unwrap().as_str().to_string())
                            }
                            else{
                                None
                            }
                        },
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
    if check(&mapped) == true{
        true
    }
    else{
        false
    }
}

fn read<R: Read>(io: R) -> Result<Vec<Vec<String>>, Error> {
    let br = BufReader::new(io);
    let mut passports: Vec<Vec<String>> = Vec::new();
    let mut passport: Vec<String> = Vec::new();
    for line in br.lines() {
        if let Ok(mut l) =  line {
            match l.is_empty() {
                true => {
                    passports.push(passport.clone());
                    passport.clear();
                }
                false => {
                    l.push('\n');
                    passport.push(l);
                },
            }
        }
    }
    passports.push(passport.clone());
    return Ok(passports);
}

fn main() -> Result<(), Error> {
    let readed: Vec<Vec<String>> = read(File::open("input")?)?;
    let mut regexes: Vec<Regex> = Vec::new();
    let mut passports:u32 = 0;
    regexes.push(Regex::new(r"(byr):([12][90]\d\d)\s").unwrap());
    regexes.push(Regex::new(r"(iyr):([2][0][12]\d)\s").unwrap());
    regexes.push(Regex::new(r"(eyr):([2][0][23]\d)\s").unwrap());
    regexes.push(Regex::new(r"(hgt):((\d{2})(in)|(\d{3})(cm))\s").unwrap());
    regexes.push(Regex::new(r"(hcl):(#[\da-f]{6})\s").unwrap());
    regexes.push(Regex::new(r"(ecl):(amb|blu|brn|gry|grn|hzl|oth)\s").unwrap());
    regexes.push(Regex::new(r"(pid):(\d{9})\s").unwrap());
    for passport in readed {
        match try_new(passport, &regexes) {
            true => passports += 1,
            false => (),
        }
    }
    println!("Resut: {}", passports);
    Ok(())
}
