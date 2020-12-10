
fn main() {
    let mut data: Vec<i64> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse::<i64>().unwrap())
        .collect();
    data.push(0);
    data.sort();
    data.push(data.last().unwrap() + 3);

    let mut ways: Vec<i64> = Vec::with_capacity(data.len());
    ways.push(1); // initial number of ways
    for idx in 1..data.len() {
        let first = (0..=idx).filter(|i| data[*i] >= data[idx] - 3).next().unwrap();
        ways.push(ways[first..idx].iter().sum());
    }

    println!("{}", ways[ways.len() - 1]);
}
