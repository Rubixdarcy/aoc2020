const PRE_ELEMENTS: usize = 25;

fn main() {
    let values: Vec<u64> = include_str!("input.txt")
        .lines()
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let ans: u64 = (PRE_ELEMENTS..values.len())
        .filter(|i| !do_two_nums_sum(values[*i], &values[i - PRE_ELEMENTS..*i]))
        .map(|i| values[i])
        .next()
        .unwrap();
    
    println!("{}", ans);
}

fn do_two_nums_sum(n: u64, nums: &[u64]) -> bool {
    for a in nums {
        for b in nums {
            if a + b == n {
                return true;
            }
        }
    }
    return false;
}
