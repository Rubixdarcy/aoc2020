use nom::{
    IResult,
    combinator::{map_res, map, value, opt},
    character::complete::{digit1, newline},
    bytes::complete::{tag,},
    multi::{many1, many0_count, many_m_n},
    branch::{alt},
    sequence::{delimited, separated_pair, preceded},
};
use std::collections::HashMap;
use std::collections::HashSet;

const BORDERS_DESCRIPTOR: [(usize, usize, M2); 4] = [
    (0, 1, M2::IDENTITY), // top
    (90, 1, M2::TX), // bottom
    (0, 10, M2::TAC), // left
    (9, 10, M2::R1) // right
];

/*
Notes:

There are 2 sections:
1. Building the complete image.
2. Finding the monsters

The representation of an image should be a HashSet<(u32, u32)> that represents the set of '#'s.

Step 1:
Create a tile descriptor that completely determines where a tile is placed


Step 2:
Create an iterator of HashSet<(u32, u32)> that yields all possible monster configurations.
Then the solution will look like:
```
monster_tiles = set()
for monster in all_monsters:
  if monster is a subset of image:
    monster_tiles = monster_tiles `union` monster

non_monster_tiles = image - monster_tiles
*/


fn main() {
    let tiles: HashMap<u32, Tile> = parse_tiles(include_str!("input.txt"))
        .unwrap().1
        .into_iter()
        .collect();

    let mut border_freq: HashMap<u16, u32> = HashMap::new();
    for (_id, t) in &tiles {
        for &border in t.borders.iter() {
            *border_freq.entry(border.0).or_insert(0) += 1;
        }
    }

    let tile_edge_count: HashMap<u32, u32> = tiles.iter()
        .map(|(&id, t)| {
            let count = t.borders.iter()
                .filter(|(b, _m)| border_freq.get(b) == Some(&1))
                .count() as u32;
            (id, count)
        })
        .collect();

    dfs(&tiles);

    //let result = tile_edge_count.iter()
    //    .filter(|&(_id, &e)| e == 2)
    //    .map(|(&id, _e)| id as u64)
    //    .product::<u64>();
    //
    //println!("{:#?}", border_freq);
}

#[derive(Copy, Clone, Hash, Debug, PartialEq)]
struct Tile {
    data: [bool; 100],
    borders: [(u16, M2); 4],
}

fn parse_tiles(i: &str) -> IResult<&str, Vec<(u32, Tile)>> {
    many1(delimited(
        many0_count(newline),
        separated_pair(
            delimited(tag("Tile "), parse_num, tag(":")),
            newline,
            parse_tile,
        ),
        many0_count(newline)
    ))(i)
}

fn parse_tile(i: &str) -> IResult<&str, Tile> {
    map(many_m_n(100, 100, preceded(opt(newline), parse_pixel)),
        |v| {
            let mut data: [bool; 100] = [false; 100];
            for i in 0..100 {
                data[i] = v[i];
            }
            let borders = calc_borders(&data);
            Tile { data, borders }
        }
    )(i)
}

fn parse_pixel(i: &str) -> IResult<&str, bool> {
    alt((
        value(false, tag(".")),
        value(true, tag("#")),
    ))(i)
}

fn calc_borders(data: &[bool]) -> [(u16, M2); 4] {
    let mut borders: [(u16, M2); 4] = [(0, M2::ZERO); 4];

    for (i, (offset, jump, tf)) in BORDERS_DESCRIPTOR.iter().copied().enumerate() {
        let mut border = (0..10)
            .map(|k| if data[offset + jump * k] { 1u16 << k } else { 0u16 })
            .sum::<u16>();
        let mut tf = tf;

        if border.reverse_bits() >> 6 < border {
            border = border.reverse_bits() >> 6;
            tf = tf * M2::TY;
        }

        borders[i] = (border, tf);
    }

    return borders;
}

fn parse_num(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct TilePosition {
    x: i32,
    y: i32,
    orientation: M2,
}

impl TilePosition {
    fn new(x: i32, y: i32, orientation: M2) -> Self {
        Self { x, y, orientation }
    }

    fn align_borders(self, my_border_m: M2, other_border_m: M2) -> Self {
        // Both pieces have the border oriented the same way, so we have:
        //     my_border_m * self.orientation
        //     = (the global border orientation)
        //     = other_border_m * other.orientation
        let border_orientation = my_border_m * self.orientation;
        let (dx, dy) = border_orientation * (0, -1);
        let (x, y) = (self.x + dx, self.y + dy);
        let orientation = other_border_m.inv() * M2::TX * border_orientation;
        Self { x, y, orientation }
    }
}

fn dfs(tiles: &HashMap<u32, Tile>) {
    let mut closed: HashSet<SearchNode> = HashSet::new();
    let mut open: Vec<SearchNode> = Vec::new();

    let first_id = *tiles.keys().next().unwrap();
    open.push(SearchNode::new(first_id, TilePosition::new(0, 0, M2::IDENTITY)));

    while open.len() > 0 {
        let node = open.pop().unwrap();
        if closed.contains(&node) { continue; }
        closed.insert(node);

        let tile = tiles.get(&node.tile_id).unwrap();

        println!("Processing node: {:?}, tile: {:?}", node, tile);

        for &(b, m) in tile.borders.iter() {
            for (&other_id, other_tile) in tiles {
                if other_id == node.tile_id { continue; }
                for &(other_b, other_m) in other_tile.borders.iter() {
                    if b == other_b {
                        let pos = node.pos.align_borders(m, other_m);
                        open.push(SearchNode::new(other_id, pos));
                        break;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct SearchNode {
    tile_id: u32,
    pos: TilePosition,
}

impl SearchNode {

    fn new(tile_id: u32, pos: TilePosition) -> Self {
        SearchNode { tile_id, pos }
    }
}

/////////////////////////////////////////////////////////
// M2
/////////////////////////////////////////////////////////
#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct M2 {
    pub a00: i32, pub a01: i32,
    pub a10: i32, pub a11: i32,
}

pub fn matrix2(a00: i32, a01: i32,
               a10: i32, a11: i32,) -> M2 {
    M2 { a00, a01,
         a10, a11, }
}

impl Eq for M2 {}

impl std::ops::Mul for M2 {
    type Output = Self;

    fn mul(self, n: Self) -> Self {
        let m = self;
        Self {
            a00: m.a00 * n.a00 + m.a01 * n.a10,
            a01: m.a00 * n.a01 + m.a01 * n.a11,
            a10: m.a10 * n.a00 + m.a11 * n.a10,
            a11: m.a10 * n.a01 + m.a11 * n.a11,
        }
    }
}

impl std::ops::Mul<(i32, i32)> for M2 {
    type Output = (i32, i32);

    fn mul(self, t: Self::Output) -> Self::Output {
        let m = self;
        (
            m.a00 * t.0 + m.a01 * t.1,
            m.a10 * t.0 + m.a11 * t.1,
        )
    }
}

impl M2 {
    pub const ZERO: Self = Self { a00: 0, a01: 0,
                                  a10: 0, a11: 0, };

    pub const IDENTITY: Self = Self { a00: 1, a01: 0,
                                      a10: 0, a11: 1, };

    // Clockwise 90
    pub const R1: Self = Self { a00: 0, a01: -1,
                                a10: 1, a11: 0, };

    pub const R2: Self = Self { a00: -1, a01: 0,
                                a10: 0, a11: -1, };

    pub const R3: Self = Self { a00: 0, a01: 1,
                                a10: -1, a11: 0, };

    pub const TX: Self = Self { a00: 1, a01: 0,
                                a10: 0, a11: -1, };

    pub const TY: Self = Self { a00: -1, a01: 0,
                                a10: 0, a11: 1, };

    pub const TAC: Self = Self { a00: 0, a01: -1,
                                 a10: 1, a11: 0, };

    pub const TBD: Self = Self { a00: 0, a01: 1,
                                 a10: -1, a11: 0, };

    pub const SIZE: usize = 2;

    pub fn transpose(self) -> Self {
        matrix2(
            self.a00, self.a10,
            self.a01, self.a11,
        )
    }

    pub fn inv(self) -> Self {
        let det = self.a00 * self.a11 - self.a01 * self.a10;
        Self {a00: self.a11 / det, a01: (-self.a01) / det,
              a10: (-self.a10) / det, a11: self.a00 / det }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tile_pos_align_borders() {
        let p1 = TilePosition::new(2, 2, M2::R1);
        let p2 = p1.align_borders(M2::R1, M2::TY);
        assert_eq!(p2, TilePosition::new(2, 3, M2::IDENTITY));
        //assert_eq!()
    }
}
