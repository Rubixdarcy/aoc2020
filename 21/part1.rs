use nom::{
    IResult,
    combinator::{peek, not},
    character::complete::{space1, alpha1},
    bytes::complete::{tag,},
    multi::{separated_list1},
    sequence::{delimited, tuple},
};
use std::collections::HashSet;
use std::collections::HashMap;

fn main() {
    let lines: Vec<(Vec<&str>, Vec<&str>)> = include_str!("input.txt")
        .lines()
        .map(|l| parse_line(l).unwrap().1)
        .collect();

    let all_ingredients: HashSet<&str> = lines.iter()
        .flat_map(|l| l.0.iter())
        .copied()
        .collect();

    let mut allergens: HashMap<&str, HashSet<&str>> = lines.iter()
        .flat_map(|l| l.1.iter())
        .copied()
        .map(|allergen| (allergen, all_ingredients.clone()))
        .collect();

    for line in lines.iter() {
        let ingredients: HashSet<&str> = line.0.iter().copied().collect();

        for allergen in line.1.iter().copied() {
            let new_set: HashSet<&str> = allergens.get(allergen).unwrap()
                .intersection(&ingredients)
                .into_iter()
                .copied()
                .collect();
            allergens.insert(allergen, new_set);
        }
    }

    let possible_allergic_ingredients: HashSet<&str> = allergens.iter()
        .flat_map(|(_, set)| set.iter().copied())
        .collect();

    let impossible_allergic_ingredients: HashSet<&str> = all_ingredients
        .difference(&possible_allergic_ingredients)
        .into_iter()
        .copied()
        .collect();

    let result = lines.iter()
        .flat_map(|l| l.0.iter())
        .copied()
        .filter(|s| impossible_allergic_ingredients.contains(s))
        .count();

    println!("{:?}", result);
}

fn parse_line(i: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    tuple((
        separated_list1(tuple((space1, peek(not(tag("("))))), alpha1),
        delimited(tag(" (contains "), separated_list1(tag(", "), alpha1), tag(")")),
    ))(i)
}
