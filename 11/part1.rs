use std::convert::TryInto;

const ADJACENTS: &[(i32, i32)] =
    &[(-1, -1), (-1, 0), (-1, 1),
      ( 0, -1),          ( 0, 1),
      ( 1, -1), ( 1, 0), ( 1, 1)];


fn main() {
    let mut layout: Vec<Vec<State>> = include_str!("input.txt").lines()
        .map(|line| {
            line.chars()
                .map(|c|
                     match c {
                         '#' => State::Full,
                         'L' => State::Empty,
                         '.' => State::Floor,
                         _ => panic!("asdf"),
                     }
                 )
                .collect::<Vec<State>>()
        })
        .collect();

    for i in 0..500 {
        layout = next_layout(&layout);
    }
    let count: usize = layout.iter().map(|row| row.iter().filter(|s| **s == State::Full).count()).sum();
    println!("{}", count);
}

fn next_layout(layout: &[Vec<State>]) -> Vec<Vec<State>> {
    let mut result: Vec<Vec<State>> = Vec::new();
    for y in 0..(layout.len() as i32) {
        let mut next_row: Vec<State> = Vec::new();
        for x in 0..(layout[y as usize].len() as i32) {
            let full_count = ADJACENTS.iter().cloned()
                .map(|(dx, dy)| (x + dx, y + dy))
                .map(|(x, y)| layout_get(layout, x, y))
                .filter(|s| *s == State::Full)
                .count();
            let current_state = layout_get(layout, x, y);

            if current_state == State::Floor {
                next_row.push(State::Floor);
                continue
            }
            if full_count == 0 {
                next_row.push(State::Full);
                continue
            }
            if full_count >= 4 {
                next_row.push(State::Empty);
                continue
            }
            next_row.push(current_state);
        }
        result.push(next_row)
    }
    return result;
}

fn layout_get(layout: &[Vec<State>], x: i32, y: i32) -> State {
    if y < 0 || y >= layout.len() as i32 {
        return State::Floor;   
    }
    let row = &layout[y as usize];

    if x < 0 || x >= row.len() as i32 {
        return State::Floor;
    }
    return row[x as usize];
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum State { Floor, Empty, Full }
