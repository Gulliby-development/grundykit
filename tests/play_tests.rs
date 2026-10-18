use grundykit::play::{self, Turn};
use grundykit::position::Part;
use grundykit::solve::Step;

fn part(text: &str) -> Part {
    Part::parse(text).unwrap()
}

#[test]
fn play_api_exposes_turn_traits_fields_and_an_empty_game() {
    fn assert_turn_traits<T: std::fmt::Debug + Clone + PartialEq + Eq>() {}

    assert_turn_traits::<Turn>();
    let input = [];
    let (turns, winner) = play::play(&input);

    assert_eq!(turns, vec![]);
    assert_eq!(winner, 'B');
    assert_eq!(turns.clone(), turns);
    assert!(format!("{turns:?}").starts_with("["));
}

#[test]
fn nim_one_two_three_matches_the_complete_literal_transcript() {
    let input = [part("nim/1"), part("nim/2"), part("nim/3")];
    let expected = vec![
        Turn {
            number: 1,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![],
                after: vec![part("nim/2"), part("nim/3")],
            },
        },
        Turn {
            number: 2,
            player: 'B',
            step: Step {
                part: 2,
                take: 1,
                leaves: vec![2],
                after: vec![part("nim/2"), part("nim/2")],
            },
        },
        Turn {
            number: 3,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![1],
                after: vec![part("nim/1"), part("nim/2")],
            },
        },
        Turn {
            number: 4,
            player: 'B',
            step: Step {
                part: 2,
                take: 1,
                leaves: vec![1],
                after: vec![part("nim/1"), part("nim/1")],
            },
        },
        Turn {
            number: 5,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![],
                after: vec![part("nim/1")],
            },
        },
        Turn {
            number: 6,
            player: 'B',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![],
                after: vec![],
            },
        },
    ];

    let (turns, winner) = play::play(&input);

    assert_eq!(turns, expected);
    assert_eq!(turns.len(), 6);
    assert_eq!(winner, 'B');
}

#[test]
fn kayles_and_nim_matches_the_complete_literal_transcript() {
    let input = [part("oct:77/5"), part("nim/2")];
    let expected = vec![
        Turn {
            number: 1,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![1, 3],
                after: vec![part("oct:77/1"), part("oct:77/3"), part("nim/2")],
            },
        },
        Turn {
            number: 2,
            player: 'B',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![],
                after: vec![part("oct:77/3"), part("nim/2")],
            },
        },
        Turn {
            number: 3,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![2],
                after: vec![part("oct:77/2"), part("nim/2")],
            },
        },
        Turn {
            number: 4,
            player: 'B',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![1],
                after: vec![part("oct:77/1"), part("nim/2")],
            },
        },
        Turn {
            number: 5,
            player: 'A',
            step: Step {
                part: 2,
                take: 1,
                leaves: vec![1],
                after: vec![part("oct:77/1"), part("nim/1")],
            },
        },
        Turn {
            number: 6,
            player: 'B',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![],
                after: vec![part("nim/1")],
            },
        },
        Turn {
            number: 7,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![],
                after: vec![],
            },
        },
    ];

    let (turns, winner) = play::play(&input);

    assert_eq!(turns, expected);
    assert_eq!(turns.len(), 7);
    assert_eq!(winner, 'A');
}

#[test]
fn empty_and_nonempty_terminal_positions_return_b_without_turns() {
    assert_eq!(play::play(&[]), (vec![], 'B'));
    assert_eq!(play::play(&[part("nim/0")]), (vec![], 'B'));
    assert_eq!(play::play(&[part("sub:2/1")]), (vec![], 'B'));
}

#[test]
fn nim_one_is_removed_by_a_and_a_wins() {
    assert_eq!(
        play::play(&[part("nim/1")]),
        (
            vec![Turn {
                number: 1,
                player: 'A',
                step: Step {
                    part: 1,
                    take: 1,
                    leaves: vec![],
                    after: vec![],
                },
            }],
            'A'
        )
    );
}

#[test]
fn two_equal_nim_heaps_have_the_complete_four_turn_losing_start() {
    assert_eq!(
        play::play(&[part("nim/2"), part("nim/2")]),
        (
            vec![
                Turn {
                    number: 1,
                    player: 'A',
                    step: Step {
                        part: 1,
                        take: 1,
                        leaves: vec![1],
                        after: vec![part("nim/1"), part("nim/2")],
                    },
                },
                Turn {
                    number: 2,
                    player: 'B',
                    step: Step {
                        part: 2,
                        take: 1,
                        leaves: vec![1],
                        after: vec![part("nim/1"), part("nim/1")],
                    },
                },
                Turn {
                    number: 3,
                    player: 'A',
                    step: Step {
                        part: 1,
                        take: 1,
                        leaves: vec![],
                        after: vec![part("nim/1")],
                    },
                },
                Turn {
                    number: 4,
                    player: 'B',
                    step: Step {
                        part: 1,
                        take: 1,
                        leaves: vec![],
                        after: vec![],
                    },
                },
            ],
            'B'
        )
    );
}

#[test]
fn zero_parts_are_preserved_when_the_second_part_moves() {
    let input = [part("nim/0"), part("nim/1"), part("nim/0")];

    assert_eq!(
        play::play(&input),
        (
            vec![Turn {
                number: 1,
                player: 'A',
                step: Step {
                    part: 2,
                    take: 1,
                    leaves: vec![],
                    after: vec![part("nim/0"), part("nim/0")],
                },
            }],
            'A'
        )
    );
}

#[test]
fn a_heap_larger_than_the_front_limit_is_played_without_a_library_limit() {
    assert_eq!(
        play::play(&[part("nim/41")]),
        (
            vec![Turn {
                number: 1,
                player: 'A',
                step: Step {
                    part: 1,
                    take: 41,
                    leaves: vec![],
                    after: vec![],
                },
            }],
            'A'
        )
    );
}

#[test]
fn repeated_split_position_calls_are_equal_and_preserve_the_input() {
    let input = [part("oct:77/5"), part("nim/2")];
    let original = input.clone();
    let first = play::play(&input);
    let second = play::play(&input);

    assert_eq!(first, second);
    assert_eq!(input, original);
    assert_eq!(
        first.0[0],
        Turn {
            number: 1,
            player: 'A',
            step: Step {
                part: 1,
                take: 1,
                leaves: vec![1, 3],
                after: vec![part("oct:77/1"), part("oct:77/3"), part("nim/2")],
            },
        }
    );
    assert_eq!(first.1, 'A');
}

#[test]
fn equal_heap_ties_choose_the_first_part_deterministically() {
    assert_eq!(
        play::play(&[part("nim/1"), part("nim/1"), part("nim/1")]),
        (
            vec![
                Turn {
                    number: 1,
                    player: 'A',
                    step: Step {
                        part: 1,
                        take: 1,
                        leaves: vec![],
                        after: vec![part("nim/1"), part("nim/1")],
                    },
                },
                Turn {
                    number: 2,
                    player: 'B',
                    step: Step {
                        part: 1,
                        take: 1,
                        leaves: vec![],
                        after: vec![part("nim/1")],
                    },
                },
                Turn {
                    number: 3,
                    player: 'A',
                    step: Step {
                        part: 1,
                        take: 1,
                        leaves: vec![],
                        after: vec![],
                    },
                },
            ],
            'A'
        )
    );
}
