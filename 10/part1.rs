
fn main() {
    let mut data: Vec<u32> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    data.sort();
    data.push(data.last().unwrap() + 3);

    let count_1 = diff_iter(&data).filter(|n| *n == 1).count();
    let count_3 = diff_iter(&data).filter(|n| *n == 3).count();

    println!("{}", count_1 * count_3); 
}

fn diff_iter<'a>(ns: &'a [u32]) -> impl Iterator<Item = u32> + 'a {
    let it_a = ns.iter().cloned();
    let it_b = (0..1).chain(ns.iter().cloned());

    return it_a
        .zip(it_b)
        .map(|(a, b)| a - b);
}
