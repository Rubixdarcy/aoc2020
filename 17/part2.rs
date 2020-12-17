use itertools::Itertools;
use std::convert::TryFrom;

const GRID_SIZE: usize = 20;
const INPUT_OFFSET: usize = 6;
const NCYCLES: usize = 6;

#[derive(Default)]
struct Grid {
    grid: [[[[bool; GRID_SIZE]; GRID_SIZE]; GRID_SIZE]; GRID_SIZE],
}

impl Grid {
    fn get(&self, x: isize, y: isize, z: isize, w: isize) -> bool {
        let x = match usize::try_from(x) { Ok(a) => a, _ => return false };
        let y = match usize::try_from(y) { Ok(a) => a, _ => return false };
        let z = match usize::try_from(z) { Ok(a) => a, _ => return false };
        let w = match usize::try_from(w) { Ok(a) => a, _ => return false };

        self.grid.get(w)
            .and_then(|cube| cube.get(z))
            .and_then(|layer| layer.get(y))
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or_default()
    }
}

fn main() {
    let mut grid = Grid::default();
    let mut grid_new = Grid::default();

    for (y, line) in include_str!("input.txt").lines().enumerate() {
        for (x, chr) in line.chars().enumerate() {
            if chr == '#' {
                grid.grid[INPUT_OFFSET][INPUT_OFFSET][y + INPUT_OFFSET][x + INPUT_OFFSET] = true;
            }
        }
    }

    for _cycle in 0..NCYCLES {
        next_grid(&grid, &mut grid_new);
        std::mem::swap(&mut grid, &mut grid_new);
    }

    let count = grid.grid.iter()
        .flat_map(|cube| cube.iter())
        .flat_map(|layer| layer.iter())
        .flat_map(|row| row.iter())
        .filter(|active| **active)
        .count();

    println!("{}", count);
}

fn next_grid(curr_grid: &Grid, next_grid: &mut Grid) {
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            for z in 0..GRID_SIZE {
                for w in 0..GRID_SIZE {
                    let active = curr_grid.get(x as isize, y as isize, z as isize, w as isize);
                    let n = neighbours(x as isize, y as isize, z as isize, w as isize)
                        .filter(|&(x, y, z, w)| curr_grid.get(x, y, z, w))
                        .count();
                    let result = match (active, n) {
                        (true, 2) => true,
                        (true, 3) => true,
                        (false, 3) => true,
                        _ => false,
                    };
                    next_grid.grid[w][z][y][x] = result;
                }
            }
        }
    }
}

fn neighbours(x: isize, y: isize, z: isize, w: isize)
        -> impl Iterator<Item = (isize, isize, isize, isize)> {
    [-1isize, 0isize, 1isize].iter().cloned()
        .cartesian_product([-1isize, 0isize, 1isize].iter().cloned())
        .cartesian_product([-1isize, 0isize, 1isize].iter().cloned())
        .cartesian_product([-1isize, 0isize, 1isize].iter().cloned())
        .map(|(((x, y), z), w)| (x, y, z, w))
        .filter(|coord| *coord != (0, 0, 0, 0))
        .map(move |(dx, dy, dz, dw)| (x + dx, y + dy, z + dz, w + dw))
}
