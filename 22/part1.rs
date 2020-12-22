use std::collections::VecDeque;

fn main() {
    let mut lines = include_str!("input.txt").lines();

    lines.next(); // "Player 1"

    let mut deck1: VecDeque<u32> = (&mut lines)
        .take_while(|l| l.len() > 0)
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    lines.next(); // "Player 2"
    let mut deck2: VecDeque<u32> = (&mut lines)
        .map(|l| l.parse::<u32>().unwrap())
        .collect();

    simulate_game(&mut deck1, &mut deck2);

    let winning_deck = if deck1.len() > 0 { &deck1 } else { &deck2 };

    let result = winning_deck.iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, c)| (i as u32 + 1) * c)
        .sum::<u32>();

    println!("{}", result);

}

fn simulate_game<'a>(deck1: &'a mut VecDeque<u32>, deck2: &'a mut VecDeque<u32>) {
    while deck1.len() > 0 && deck2.len() > 0 {
        let c1 = deck1.pop_front().unwrap();
        let c2 = deck2.pop_front().unwrap();
        if c1 > c2 {
            deck1.push_back(c1);
            deck1.push_back(c2);
        } else {
            deck2.push_back(c2);
            deck2.push_back(c1);
        }
    }
}
