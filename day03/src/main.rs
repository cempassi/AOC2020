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
    tree_seen: u64,
}

struct Map {
    map: Vec<Vec<char>>,
    width: usize,
    height: usize,
}

impl Point {
    fn moving(self: Self, direction: &Point) -> Point {
        Point {
            line: self.line + direction.line,
            column: self.column + direction.column,
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

    fn move_once(self: &mut Self, map: &Map, direction: &Point) {
        self.position = self.position.moving(direction);
        match map.translate(&self.position) {
            '#' => self.tree_seen += 1,
            _ => (),
        };
    }
}

fn travel(map: &Map, direction: Point) -> u64 {
    let mut traveler = Traveler {
        position: Point { line: 0, column: 0 },
        tree_seen: 0,
    };

    while traveler.is_traveling(map) {
        traveler.move_once(map, &direction);
    }
    println!("Number of trees: {}", traveler.tree_seen);
    traveler.tree_seen
}

fn main() -> Result<(), Error> {
    let readed: Vec<Vec<char>> = read(File::open("input")?)?;
    let map: Map = Map {
        width: readed[0].len(),
        height: readed.len(),
        map: readed,
    };

    let mut results: Vec<u64> = Vec::new();
    results.push(travel(&map, Point{line: 1, column:1}));
    results.push(travel(&map, Point{line: 1, column:3}));
    results.push(travel(&map, Point{line: 1, column:5}));
    results.push(travel(&map, Point{line: 1, column:7}));
    results.push(travel(&map, Point{line: 2, column:1}));
    let product = results.into_iter().fold(1, |acc, x| acc * x);
    println!("Product: {}", product);
    Ok(())
}
