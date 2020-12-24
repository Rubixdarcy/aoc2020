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

const BORDERS_OFFSET_JUMP: [(usize, usize); 4] = [(0, 1), (90, 1), (0, 10), (9, 10)];

fn main() {
    let tiles: HashMap<u32, Tile> = parse_tiles(include_str!("input.txt"))
        .unwrap().1
        .into_iter()
        .collect();

    let mut border_freq: HashMap<u16, u32> = HashMap::new();
    for (_id, t) in &tiles {
        for &border in t.borders.iter() {
            *border_freq.entry(border).or_insert(0) += 1;
        }
    }

    let tile_edge_count: HashMap<u32, u32> = tiles.iter()
        .map(|(&id, t)| {
            let count = t.borders.iter()
                .filter(|b| border_freq.get(b) == Some(&1))
                .count() as u32;
            (id, count)
        })
        .collect();

    let result = tile_edge_count.iter()
        .filter(|&(_id, &e)| e == 2)
        .map(|(&id, _e)| id as u64)
        .product::<u64>();
    
    println!("{:#?}", result);
}

#[derive(Copy, Clone, Hash, Debug, PartialEq)]
struct Tile {
    data: [bool; 100],
    borders: [u16; 4],
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

fn calc_borders(data: &[bool]) -> [u16; 4] {
    let mut borders: [u16; 4] = [0; 4];

    for (i, (offset, jump)) in BORDERS_OFFSET_JUMP.iter().copied().enumerate() {
        let border = (0..10)
            .map(|k| if data[offset + jump * k] { 1u16 << k } else { 0u16 })
            .sum::<u16>();
        borders[i] = border.min(border.reverse_bits() >> 6);
    }

    return borders;
}

fn parse_num(i: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calc_borders_test() {
        // Based on Tile 3559
        let data: [bool; 100] = [
            true,  true,  false, false, true,  true,  false, true,  false, false,
            true,  false, false, false, false, false, false, false, false, false,
            false, false, true,  false, false, false, false, false, false, true,
            false, false, true,  true,  false, false, false, false, false, true,
            true,  false, false, true,  false, false, true,  false, false, true,
            true,  true,  true,  true,  true,  true,  false, false, false, true,
            true,  false, false, false, false, false, true,  false, false, true,
            false, false, false, false, false, true,  false, false, false, false,
            true,  false, false, false, false, false, false, false, false, false,
            true,  true,  false, false, true,  false, false, true,  false, true,
        ];
        assert_eq!(calc_borders(&data), [
            0b10110011, // top (r)
            0b1010010011, // bottom (r)
            0b1100111011, // left
            0b11111001, // right
        ]);
    }

    #[test]
    fn bit_manipulation() {
        let border: u16 = 0b1010010011;
        
        assert_eq!(border.reverse_bits(), 0b1100100101000000);
        assert_eq!(border.reverse_bits() >> 6, 0b1100100101);
        assert_eq!(border.min(border.reverse_bits() >> 6), 0b1010010011);
    }
}
