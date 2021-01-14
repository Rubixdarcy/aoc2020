use std::collections::HashSet;
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

    let (winner, state) = simulate_game(State { deck1, deck2 });

    let winning_deck = match winner {
        Winner::P1 => &state.deck1,
        Winner::P2 => &state.deck2,
    };

    let result = winning_deck.iter()
        .rev()
        .copied()
        .enumerate()
        .map(|(i, c)| (i as u32 + 1) * c)
        .sum::<u32>();

    println!("{}", result);

}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    deck1: VecDeque<u32>,
    deck2: VecDeque<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Winner { P1, P2 }

fn simulate_game(mut state: State) -> (Winner, State) {
    let mut prev_states: HashSet<State> = HashSet::new();


    loop {
        //println!("{:?}", state);
        if state.deck1.len() == 0 { return (Winner::P2, state); }
        if state.deck2.len() == 0 { return (Winner::P1, state); }
        if prev_states.contains(&state) { return (Winner::P1, state); }

        prev_states.insert(state.clone());

        let c1 = state.deck1.pop_front().unwrap();
        let c2 = state.deck2.pop_front().unwrap();

        let round_winner = if state.deck1.len() as u32 >= c1 && state.deck2.len() as u32 >= c2 {
            // Run subgame
            let deck1: VecDeque<u32> = state.deck1.iter().take(c1 as usize).copied().collect();
            let deck2: VecDeque<u32> = state.deck2.iter().take(c2 as usize).copied().collect();
            //println!("Subgame...");
            simulate_game(State { deck1, deck2 }).0
        } else {
            if c1 > c2 { Winner::P1 } else { Winner::P2 }
        };

        match round_winner {
            Winner::P1 => {
                state.deck1.push_back(c1);
                state.deck1.push_back(c2);
            },
            Winner::P2 => {
                state.deck2.push_back(c2);
                state.deck2.push_back(c1);
            },
        }
    }
}
