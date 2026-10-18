use crate::rule::Rule;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub take: u32,
    pub leaves: Vec<u32>,
}

pub fn of(rule: &Rule, n: u32) -> Vec<Move> {
    if n == 0 {
        return Vec::new();
    }

    match rule {
        Rule::Nim => (1..=n)
            .map(|take| Move {
                take,
                leaves: positive_remainder(n - take),
            })
            .collect(),
        Rule::Sub(takes) => takes
            .iter()
            .copied()
            .filter(|take| *take <= n)
            .map(|take| Move {
                take,
                leaves: positive_remainder(n - take),
            })
            .collect(),
        Rule::Oct(digits) => {
            let mut moves = Vec::new();
            for (index, digit) in digits.iter().copied().enumerate() {
                let take = index as u32 + 1;
                if take > n {
                    break;
                }
                let remainder = n - take;

                if digit & 1 != 0 && remainder == 0 {
                    moves.push(Move {
                        take,
                        leaves: Vec::new(),
                    });
                }
                if digit & 2 != 0 && remainder >= 1 {
                    moves.push(Move {
                        take,
                        leaves: vec![remainder],
                    });
                }
                if digit & 4 != 0 && remainder >= 2 {
                    for smaller in 1..=remainder / 2 {
                        moves.push(Move {
                            take,
                            leaves: vec![smaller, remainder - smaller],
                        });
                    }
                }
            }
            moves
        }
    }
}

fn positive_remainder(remainder: u32) -> Vec<u32> {
    if remainder == 0 {
        Vec::new()
    } else {
        vec![remainder]
    }
}
