use std::collections::HashMap;

const MY_BAG: &str = "shiny gold";

fn main() {
    let raw_input = include_str!("input.txt");
    let contains_list = parse::bag_data(raw_input).unwrap().1;
    println!("{}", count(MY_BAG, &contains_list, &mut HashMap::new()));
}

fn count<'a>(bag: &'a str, contains_list: &Vec<(&'a str, Vec<(u32, &'a str)>)>, count_map: &mut HashMap<&'a str, u32>) -> u32 {
    if let Some(n) = count_map.get(bag) {
        return *n;
    }

    let item = contains_list.iter().find(|(s, _)| *s == bag).unwrap();
    let result: u32 = item.1.iter()
        .map(|(n, s)| n + n * count(s, contains_list, count_map))
        .sum();
    count_map.insert(bag, result);
    return result;
}


mod parse {
    use nom::{
      IResult,
      character::complete::{space0, digit0, newline},
      bytes::complete::{tag, take_until},
      combinator::{opt, map, map_res},
      sequence::{terminated, tuple, separated_pair},
      branch::{alt},
      multi::{separated_list1},
    };

    fn bag_phrase(i: &str) -> IResult<&str, &str> {
        terminated(take_until(" bag"), tuple((tag(" bag"), opt(tag("s")))))(i)
    }

    fn int_bag_phrase(i: &str) -> IResult<&str, (u32, &str)> {
        separated_pair(map_res(digit0, |s: &str| s.parse::<u32>()), space0, bag_phrase)(i)
    }

    fn bag_line(i: &str) -> IResult<&str, (&str, Vec<(u32, &str)>)> {
        terminated(
            separated_pair(bag_phrase, tag(" contain "), alt((
                map(tag("no other bags"), |_: &str| Vec::new()),
                separated_list1(tag(", "), int_bag_phrase)))),
            tag("."))(i)
    }

    pub fn bag_data(i: &str) -> IResult<&str, Vec<(&str, Vec<(u32, &str)>)>> {
        separated_list1(newline, bag_line)(i)
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn test_bag_phrase() {
            let input = "posh blue bags contain 5";
            let (i, phrase) = bag_phrase(input).unwrap();
            assert_eq!("posh blue", phrase);
            assert_eq!(" contain 5", i);
        }
        #[test]
        fn test_int_bag_phrase() {
            let input = "3 red bags XXX";
            let (i, phrase) = int_bag_phrase(input).unwrap();
            assert_eq!((3, "red"), phrase);
            assert_eq!(" XXX", i);
        }
        #[test]
        fn test_bag_line() {
            let input = "vibrant brown bags contain 5 dark olive bags, 4 pale salmon bags.";
            let (_i, line) = bag_line(input).unwrap();
            assert_eq!(("vibrant brown", vec![(5, "dark olive"), (4, "pale salmon")]), line);
        }

    }



    //fn bag(i: &str) -> IResult<&str, &str> {
    //    let bag_name_word = terminated(take_while(is_alphabetic), take_while(space));
    //}
}
