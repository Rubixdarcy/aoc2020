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
use itertools::Itertools;

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

    let allergen_list: Vec<&str> = allergens.keys().copied().collect();
    let mut exact_allergens: Vec<(&str, &str)> = Vec::new();

    loop {
        let mut flag = false;
        for a in allergen_list.iter().copied() {
            let set = allergens.get(a).unwrap();
            if set.len() == 1 {
                let ingredient = set.iter().copied().next().unwrap();
                exact_allergens.push((a, ingredient));
                for (_, is) in allergens.iter_mut() {
                    is.remove(ingredient);
                }
                flag = true;
                break;
            }
        }
        if !flag { break; }
    }

    exact_allergens.sort_by_key(|&(a, _i)| a);

    println!("{}", exact_allergens.iter().map(|(_a, i)| i).join(","));
}

fn parse_line(i: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    tuple((
        separated_list1(tuple((space1, peek(not(tag("("))))), alpha1),
        delimited(tag(" (contains "), separated_list1(tag(", "), alpha1), tag(")")),
    ))(i)
}
