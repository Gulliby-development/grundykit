use grundykit::period::detect;
use grundykit::position::Part;
use grundykit::rule::Rule;
use grundykit::solve::{self, Step};
use grundykit::table::{grundy, mex};
use std::time::{Duration, Instant};

#[test]
fn mex_of_an_empty_slice_is_zero() {
    assert_eq!(mex(&[]), 0);
}

#[test]
fn mex_of_a_slice_containing_zero_is_one() {
    assert_eq!(mex(&[0]), 1);
}

#[test]
fn mex_can_be_zero_when_the_slice_starts_above_zero() {
    assert_eq!(mex(&[1, 2]), 0);
}

#[test]
fn mex_ignores_duplicates_when_the_initial_values_are_present() {
    assert_eq!(mex(&[2, 0, 2, 1]), 3);
}

#[test]
fn mex_ignores_the_largest_u64_without_indexing_by_its_value() {
    assert_eq!(mex(&[0, u64::MAX]), 1);
}

#[test]
fn nim_grundy_table_includes_the_zero_heap_at_maximum_zero() {
    assert_eq!(grundy(&Rule::Nim, 0), vec![0]);
}

#[test]
fn nim_grundy_table_is_the_heap_size_through_the_inclusive_maximum() {
    assert_eq!(grundy(&Rule::Nim, 5), vec![0, 1, 2, 3, 4, 5]);
}

#[test]
fn subtraction_grundy_table_uses_only_takes_not_above_the_heap() {
    assert_eq!(
        grundy(&Rule::Sub(vec![2, 3]), 9),
        vec![0, 0, 1, 1, 2, 0, 0, 1, 1, 2]
    );
}

#[test]
fn kayles_grundy_table_matches_the_literal_twenty_value_sequence() {
    assert_eq!(
        grundy(&Rule::Oct(vec![7, 7]), 19),
        vec![0, 1, 2, 3, 1, 4, 3, 2, 1, 4, 2, 6, 4, 1, 2, 7, 1, 4, 3, 2]
    );
}

#[test]
fn dawsons_kayles_grundy_table_matches_the_literal_sequence() {
    assert_eq!(
        grundy(&Rule::Oct(vec![0, 7]), 10),
        vec![0, 0, 1, 1, 2, 0, 3, 1, 1, 0, 3]
    );
}

#[test]
fn octal_one_bit_allows_a_no_heap_outcome() {
    assert_eq!(grundy(&Rule::Oct(vec![1]), 1), vec![0, 1]);
}

#[test]
fn octal_two_bit_allows_a_single_heap_outcome() {
    assert_eq!(grundy(&Rule::Oct(vec![2]), 3), vec![0, 0, 1, 0]);
}

#[test]
fn octal_four_bit_allows_split_outcomes() {
    assert_eq!(grundy(&Rule::Oct(vec![4]), 5), vec![0, 0, 0, 1, 1, 2]);
}

#[test]
fn octal_six_bit_combines_single_heap_and_split_outcomes() {
    assert_eq!(grundy(&Rule::Oct(vec![6]), 3), vec![0, 0, 1, 2]);
}

#[test]
fn dense_octal_table_at_the_maximum_has_literal_entries_and_detectable_period() {
    let values = grundy(&Rule::Oct(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]), 5000);

    assert_eq!(values.len(), 5001);
    assert_eq!(values[0], 0);
    assert_eq!(values[1], 1);
    assert_eq!(values[2], 2);
    assert_eq!(values[3], 3);
    assert_eq!(values[4], 4);
    assert!(detect(&values).is_some());
}

#[test]
fn solve_step_exposes_its_public_fields_and_required_traits() {
    fn assert_traits<T: std::fmt::Debug + Clone + PartialEq + Eq>() {}

    assert_traits::<Step>();
    let step = solve::first(&[solve_part("nim/1")]).unwrap();

    assert_eq!(step.part, 1);
    assert_eq!(step.take, 1);
    assert_eq!(step.leaves, vec![]);
    assert_eq!(step.after, vec![]);
    assert_eq!(step.clone(), step);
    assert!(format!("{step:?}").starts_with("Step"));
}

fn solve_part(text: &str) -> Part {
    Part::parse(text).unwrap()
}

#[test]
// Includes the literal AC-002 position nim/0 and AC-003 position nim/0 sub:2/1.
fn solve_value_of_empty_and_immovable_positions_is_zero() {
    assert_eq!(solve::value(&[]), 0);
    assert_eq!(solve::value(&[solve_part("nim/0")]), 0);
    assert_eq!(solve::value(&[solve_part("sub:2/1")]), 0);
    assert_eq!(
        solve::value(&[solve_part("nim/0"), solve_part("sub:2/1")]),
        0
    );
}

#[test]
// AC-002 positions: nim/3 nim/4 nim/5 and nim/2000.
fn solve_value_xors_same_rule_heap_values_and_includes_the_boundary_size() {
    assert_eq!(
        solve::value(&[
            solve_part("nim/3"),
            solve_part("nim/4"),
            solve_part("nim/5")
        ]),
        2
    );
    assert_eq!(solve::value(&[solve_part("nim/2000")]), 2000);
}

#[test]
// AC-002 mixed position: oct:77/9 nim/6 sub:1,3,4/7.
fn solve_value_xors_mixed_rule_heap_values() {
    assert_eq!(
        solve::value(&[
            solve_part("oct:77/9"),
            solve_part("nim/6"),
            solve_part("sub:1,3,4/7")
        ]),
        2
    );
}

#[test]
// AC-003 positions: empty, nim/0 sub:2/1, and nim/2 nim/2.
fn solve_winning_and_first_return_none_for_empty_and_terminal_positions() {
    assert_eq!(solve::winning(&[]), None);
    assert_eq!(solve::first(&[]), None);
    assert_eq!(
        solve::winning(&[solve_part("nim/0"), solve_part("sub:2/1")]),
        None
    );
    assert_eq!(
        solve::first(&[solve_part("nim/0"), solve_part("sub:2/1")]),
        None
    );
    assert_eq!(
        solve::winning(&[solve_part("nim/2"), solve_part("nim/2")]),
        None
    );
}

#[test]
fn solve_first_finds_the_first_move_even_in_a_losing_position() {
    let parts = [solve_part("nim/2"), solve_part("nim/2")];

    assert_eq!(
        solve::first(&parts),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![1],
            after: vec![solve_part("nim/1"), solve_part("nim/2")]
        })
    );
}

#[test]
// AC-003 position: nim/1 nim/1 nim/1.
fn solve_winning_uses_the_first_winning_nim_move_deterministically() {
    let parts = [
        solve_part("nim/1"),
        solve_part("nim/1"),
        solve_part("nim/1"),
    ];
    let expected_winning = Some(Step {
        part: 1,
        take: 1,
        leaves: vec![],
        after: vec![solve_part("nim/1"), solve_part("nim/1")],
    });
    let expected_first = Some(Step {
        part: 1,
        take: 1,
        leaves: vec![],
        after: vec![solve_part("nim/1"), solve_part("nim/1")],
    });

    assert_eq!(solve::winning(&parts), expected_winning);
    assert_eq!(solve::winning(&parts), expected_winning);
    assert_eq!(solve::first(&parts), expected_first);
    assert_eq!(solve::first(&parts), expected_first);
}

#[test]
// AC-004 first: nim/0 sub:2/1 nim/2 nim/0 | 3 | nim/0 sub:2/1 nim/1 nim/0
fn solve_first_skips_immovable_parts_and_preserves_surrounding_zeros() {
    let parts = [
        solve_part("nim/0"),
        solve_part("sub:2/1"),
        solve_part("nim/2"),
        solve_part("nim/0"),
    ];

    assert_eq!(
        solve::first(&parts),
        Some(Step {
            part: 3,
            take: 1,
            leaves: vec![1],
            after: vec![
                solve_part("nim/0"),
                solve_part("sub:2/1"),
                solve_part("nim/1"),
                solve_part("nim/0")
            ]
        })
    );
}

#[test]
// AC-003 position: nim/3 nim/4 nim/5.
fn solve_selects_ascending_takes_for_first_and_winning_nim_moves() {
    let parts = [
        solve_part("nim/3"),
        solve_part("nim/4"),
        solve_part("nim/5"),
    ];

    assert_eq!(
        solve::winning(&parts),
        Some(Step {
            part: 1,
            take: 2,
            leaves: vec![1],
            after: vec![
                solve_part("nim/1"),
                solve_part("nim/4"),
                solve_part("nim/5")
            ]
        })
    );
    assert_eq!(
        solve::first(&parts),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![2],
            after: vec![
                solve_part("nim/2"),
                solve_part("nim/4"),
                solve_part("nim/5")
            ]
        })
    );
}

#[test]
// AC-003 position: oct:77/5.
fn solve_orders_kayles_moves_and_returns_the_winning_split() {
    let parts = [solve_part("oct:77/5")];

    assert_eq!(
        solve::first(&parts),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![4],
            after: vec![solve_part("oct:77/4")]
        })
    );
    assert_eq!(
        solve::winning(&parts),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![2, 2],
            after: vec![solve_part("oct:77/2"), solve_part("oct:77/2")]
        })
    );
}

#[test]
// AC-003 position: oct:77/9 nim/6 sub:1,3,4/7.
// AC-004 winning: oct:77/9 nim/6 sub:1,3,4/7 | 1 | oct:77/2 oct:77/5 nim/6 sub:1,3,4/7
// AC-003 position: oct:77/9 nim/6 sub:1,3,4/7.
fn solve_mixed_rule_selection_matches_the_literal_winning_and_first_steps() {
    let parts = [
        solve_part("oct:77/9"),
        solve_part("nim/6"),
        solve_part("sub:1,3,4/7"),
    ];

    assert_eq!(
        solve::winning(&parts),
        Some(Step {
            part: 1,
            take: 2,
            leaves: vec![2, 5],
            after: vec![
                solve_part("oct:77/2"),
                solve_part("oct:77/5"),
                solve_part("nim/6"),
                solve_part("sub:1,3,4/7")
            ]
        })
    );
    assert_eq!(
        solve::first(&parts),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![8],
            after: vec![
                solve_part("oct:77/8"),
                solve_part("nim/6"),
                solve_part("sub:1,3,4/7")
            ]
        })
    );
}

#[test]
// AC-004 operation and input | part index | after
// winning: nim/0 nim/1 sub:2/1 | 2 | nim/0 sub:2/1
fn solve_replaces_a_heap_in_place_between_untouched_parts() {
    let parts = [
        solve_part("nim/0"),
        solve_part("nim/1"),
        solve_part("sub:2/1"),
    ];

    assert_eq!(
        solve::winning(&parts),
        Some(Step {
            part: 2,
            take: 1,
            leaves: vec![],
            after: vec![solve_part("nim/0"), solve_part("sub:2/1")]
        })
    );
}

#[test]
// AC-004 winning: nim/0 oct:77/5 nim/0 | 2 | nim/0 oct:77/2 oct:77/2 nim/0
fn solve_replaces_a_heap_with_ordered_split_leaves_and_keeps_zero_parts() {
    let parts = [
        solve_part("nim/0"),
        solve_part("oct:77/5"),
        solve_part("nim/0"),
    ];

    assert_eq!(
        solve::winning(&parts),
        Some(Step {
            part: 2,
            take: 1,
            leaves: vec![2, 2],
            after: vec![
                solve_part("nim/0"),
                solve_part("oct:77/2"),
                solve_part("oct:77/2"),
                solve_part("nim/0")
            ]
        })
    );
}

#[test]
// AC-004 winning: nim/1 | 1 | empty
fn solve_removes_a_heap_when_the_selected_move_leaves_no_heaps() {
    assert_eq!(
        solve::winning(&[solve_part("nim/1")]),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![],
            after: vec![]
        })
    );
}

#[test]
fn solve_calls_do_not_mutate_the_input_position() {
    let parts = [
        solve_part("nim/0"),
        solve_part("oct:77/5"),
        solve_part("sub:1,3,4/7"),
    ];
    let original = parts.clone();

    let _ = solve::value(&parts);
    let _ = solve::winning(&parts);
    let _ = solve::first(&parts);

    assert_eq!(parts, original);
}

#[test]
fn solve_shares_equal_rule_tables_through_the_largest_required_heap() {
    assert_eq!(
        solve::value(&[solve_part("sub:2,3/1"), solve_part("sub:2,3/1999")]),
        2
    );
}

#[test]
fn solve_handles_eight_dense_octal_boundary_parts_within_ten_seconds() {
    let parts = [
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
        solve_part("oct:777777777777/2000"),
    ];

    let started = Instant::now();
    assert_eq!(solve::value(&parts), 0);
    assert!(started.elapsed() < Duration::from_secs(10));

    let started = Instant::now();
    assert_eq!(solve::winning(&parts), None);
    assert!(started.elapsed() < Duration::from_secs(10));
}

#[test]
fn solve_evaluates_a_nonzero_size_2000_winning_candidate_within_ten_seconds() {
    let parts = [solve_part("nim/2000"), solve_part("nim/1999")];

    let started = Instant::now();
    assert_eq!(solve::value(&parts), 31);
    assert!(started.elapsed() < Duration::from_secs(10));

    let started = Instant::now();
    assert_eq!(
        solve::winning(&parts),
        Some(Step {
            part: 1,
            take: 1,
            leaves: vec![1999],
            after: vec![solve_part("nim/1999"), solve_part("nim/1999")]
        })
    );
    assert!(started.elapsed() < Duration::from_secs(10));
}
