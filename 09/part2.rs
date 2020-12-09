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

    let ans_range = (0..values.len())
        .filter_map(|i| contiguous_prefix_sum(ans, &values[i..]))
        .next()
        .unwrap();

    let min = ans_range.iter().min().unwrap();
    let max = ans_range.iter().max().unwrap();
    
    println!("{}", min + max);
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

fn contiguous_prefix_sum(n: u64, nums: &[u64]) -> Option<&[u64]> {
    let mut total = 0u64;
    
    for i in 0..nums.len() {
        total += nums[i];
        if total == n {
            return Some(&nums[0..i]);
        }
        if total > n {
            break;
        }
    }
    return None;
}
