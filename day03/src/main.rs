use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader, Error};

fn read<R: Read>(io: R) -> Result<Vec<Vec<char>>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| Ok(v.chars().collect())))
        .collect()
}

#[derive(Copy, Clone)]
struct Point {
    line: usize,
    column: usize,
}

struct Traveler {
    position: Point,
    tree_seen: u32,
}

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Point {
    fn moving(self: Self, line: usize, column: usize) -> Point {
        Point {
            line: self.line + line,
            column: self.column + column,
        }
    }
}

impl Map {
    fn translate(self: &Self, point: &Point) -> char {
        let translated_column = point.column % self.width;
        self.map[point.line][translated_column]
    }
}

impl Traveler {
    fn is_traveling(self: &Self, map: &Map) -> bool {
        if self.position.line == map.height - 1 {
            false
        } else {
            true
        }
    }

    fn move_once(self: &mut Self, map: &Map) {
        self.position = self.position.moving(1, 3);
        match map.translate(&self.position) {
            '#' => self.tree_seen += 1,
            _ => (),
        };
    }
}

fn travel(map: Map, mut traveler: Traveler) {
    while traveler.is_traveling(&map) {
        traveler.move_once(&map);
    }
    println!("Number of trees: {}", traveler.tree_seen);
}

fn main() -> Result<(), Error> {
    let readed: Vec<Vec<char>> = read(File::open("input")?)?;
    let map: Map = Map {
        width: readed[0].len(),
        height: readed.len(),
        map: readed,
    };
    let traveler = Traveler {
        position: Point { line: 0, column: 0 },
        tree_seen: 0,
    };
    travel(map, traveler);
    Ok(())
}
