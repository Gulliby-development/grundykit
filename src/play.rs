use crate::position::Part;
use crate::solve::{self, Step};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub number: usize,
    pub player: char,
    pub step: Step,
}

pub fn play(parts: &[Part]) -> (Vec<Turn>, char) {
    let mut current = parts.to_vec();
    let mut turns = Vec::new();
    let mut player = 'A';

    loop {
        let Some(step) = solve::winning(&current).or_else(|| solve::first(&current)) else {
            let winner = if player == 'A' { 'B' } else { 'A' };
            return (turns, winner);
        };

        current = step.after.clone();
        turns.push(Turn {
            number: turns.len() + 1,
            player,
            step,
        });
        player = if player == 'A' { 'B' } else { 'A' };
    }
}
