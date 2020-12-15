use std::collections::HashMap;

fn main() {
    let ns: Vec<u32> = include_str!("input.txt")
        .lines()
        .next()
        .unwrap()
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let mut last_seen: HashMap<u32, u32> = HashMap::new();

    // Initial insert (NOTE turns start at 1)
    for (i, n) in ns.iter().enumerate() {
        //last_seen.insert(n, 0);
        last_seen.insert(*n, (i + 1) as u32);
    }

    let start_turn = (ns.len() as u32) + 1;

    let mut turn_result = ns[ns.len() - 1];
    let mut next_turn_result = 0;

    for turn in start_turn..=30_000_000_u32 {
        turn_result = next_turn_result;
        
        if let Some(last_seen_turn) = last_seen.get(&turn_result) {
            next_turn_result = turn - last_seen_turn;
        } else {
            next_turn_result = 0;
        }

        last_seen.insert(turn_result, turn);
    }

    println!("{}", turn_result);

    //turn_result = 6;
    //turn_result_age = 0;
    //last_seen 
    
}
