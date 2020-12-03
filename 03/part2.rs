
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::str;
use std::iter::Iterator;

fn main() {
    let data: Vec<String> = get_input_lines("input.txt").collect();
    let map = Map::from_strings(data);

    let mut n: u64 = calculate_result(&map, 1, 1) as u64;
    n *= calculate_result(&map, 3, 1) as u64;
    n *= calculate_result(&map, 5, 1) as u64;
    n *= calculate_result(&map, 7, 1) as u64;
    n *= calculate_result(&map, 1, 2) as u64;

    println!("{}", n);
}

fn calculate_result(map: &Map, jx: u32, jy: u32) -> u32 {
    (0..map.height() / jy).into_iter()
        .map(|i| map.get_cell(i * jx, i * jy))
        .filter(|c| *c == Cell::Tree)
        .count() as u32
}

#[derive(Debug, PartialEq)]
enum Cell { Empty, Tree, }

#[derive(Debug)]
struct Map {
    grid: Vec<Vec<u8>>,
}

impl Map {
    fn from_strings(lines: Vec<String>) -> Self {
        Map {
            grid: lines.into_iter()
                .map(|l| l.into_bytes())
                .collect::<Vec<Vec<u8>>>(),
        }
    }

    fn height(&self) -> u32 { self.grid.len() as u32 }

    fn get_cell(&self, x: u32, y: u32) -> Cell {
        let row = &self.grid[y as usize];
        let cell = row[(x % row.len() as u32) as usize];

        if cell == '.' as u8 {
            return Cell::Empty;
        }
        if cell == '#' as u8 {
            return Cell::Tree;
        }
        panic!("Unknown cell character: '{}'", cell);
    }
}


fn get_input_lines(filename: &str) -> impl Iterator<Item = String> {
    return BufReader::new(File::open(filename).unwrap())
        .lines()
        .map(|line| line.unwrap());
}
