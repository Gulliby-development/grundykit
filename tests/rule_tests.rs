use grundykit::error::GameError;
use grundykit::moves::{self, Move};
use grundykit::position::{self, Part};
use grundykit::rule::Rule;

#[test]
fn nim_rule_parses_and_names_canonically() {
    assert_eq!(Rule::parse("nim"), Ok(Rule::Nim));
    assert_eq!(Rule::Nim.name(), "nim");
    assert_eq!(Rule::parse(&Rule::Nim.name()), Ok(Rule::Nim));
}

#[test]
fn subtraction_rule_sorts_and_deduplicates_takes() {
    let rule = Rule::parse("sub:4,1,3,3").expect("valid subtraction rule");
    assert_eq!(rule, Rule::Sub(vec![1, 3, 4]));
    assert_eq!(rule.name(), "sub:1,3,4");
    assert_eq!(Rule::parse(&rule.name()), Ok(rule));
}

#[test]
fn subtraction_rule_accepts_inclusive_take_bounds_and_eight_inputs() {
    assert_eq!(Rule::parse("sub:1000"), Ok(Rule::Sub(vec![1000])));
    assert_eq!(Rule::parse("sub:1,1,1,1,1,1,1,1"), Ok(Rule::Sub(vec![1])));
    assert_eq!(
        Rule::parse("sub:1,2,3,4,5,6,7,1000"),
        Ok(Rule::Sub(vec![1, 2, 3, 4, 5, 6, 7, 1000]))
    );
}

#[test]
fn octal_rule_preserves_numeric_digits_and_leading_zeroes() {
    let rule = Rule::parse("oct:07").expect("valid octal rule");
    assert_eq!(rule, Rule::Oct(vec![0, 7]));
    assert_eq!(rule.name(), "oct:07");
    assert_eq!(Rule::parse(&rule.name()), Ok(rule));
}

#[test]
fn octal_rule_accepts_twelve_digits_and_preserves_them() {
    let rule = Rule::parse("oct:777777777777").expect("valid octal rule");
    assert_eq!(rule, Rule::Oct(vec![7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7]));
    assert_eq!(rule.name(), "oct:777777777777");
}

#[test]
fn rule_variants_are_cloneable_debuggable_and_comparable() {
    let rule = Rule::Sub(vec![2, 5]);
    assert_eq!(rule.clone(), Rule::Sub(vec![2, 5]));
    assert_ne!(rule, Rule::Sub(vec![2, 6]));
    assert_eq!(format!("{rule:?}"), "Sub([2, 5])");
}

#[test]
fn malformed_rule_syntax_has_the_exact_diagnostic() {
    assert_eq!(
        Rule::parse(""),
        Err(GameError(
            "invalid rule \"\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:"),
        Err(GameError(
            "invalid rule \"sub:\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:1,,2"),
        Err(GameError(
            "invalid rule \"sub:1,,2\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:01"),
        Err(GameError(
            "invalid rule \"sub:01\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
}

#[test]
fn malformed_subtraction_tokens_reject_signs_spaces_and_non_ascii_digits() {
    assert_eq!(
        Rule::parse("sub:+1"),
        Err(GameError(
            "invalid rule \"sub:+1\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:1 2"),
        Err(GameError(
            "invalid rule \"sub:1?2\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:١"),
        Err(GameError(
            "invalid rule \"sub:?\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
}

#[test]
fn complete_subtraction_syntax_is_checked_before_the_take_count() {
    // Diagnostic class: syntax; the original TEXT is quoted after complete syntax validation.
    // The syntax error in sub:0,+1 beats the earlier invalid take range for 0.
    assert_eq!(
        Rule::parse("sub:1,2,3,4,5,6,7,8,9x"),
        Err(GameError(
            "invalid rule \"sub:1,2,3,4,5,6,7,8,9x\" (expected nim, sub:1,3,4 or oct:77)"
                .to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:0,+1"),
        Err(GameError(
            "invalid rule \"sub:0,+1\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
}

#[test]
fn subtraction_take_count_precedes_range_validation() {
    // A take count error takes precedence over the first take range error.
    assert_eq!(
        Rule::parse("sub:1,2,3,4,5,6,7,8,1001"),
        Err(GameError(
            "too many takes in \"sub:1,2,3,4,5,6,7,8,1001\" (at most 8)".to_owned()
        ))
    );
}

#[test]
fn subtraction_reports_the_first_invalid_take_from_left_to_right() {
    assert_eq!(
        Rule::parse("sub:1001,0"),
        Err(GameError(
            "invalid take \"1001\" in \"sub:1001,0\" (expected 1 to 1000)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:0,1001"),
        Err(GameError(
            "invalid take \"0\" in \"sub:0,1001\" (expected 1 to 1000)".to_owned()
        ))
    );
}

#[test]
fn subtraction_rejects_a_twenty_digit_canonical_take_without_overflow() {
    assert_eq!(
        Rule::parse("sub:12345678901234567890"),
        Err(GameError(
            "invalid take \"12345678901234567890\" in \"sub:12345678901234567890\" (expected 1 to 1000)"
                .to_owned()
        ))
    );
}

#[test]
fn subtraction_rejects_zero_and_takes_above_one_thousand() {
    assert_eq!(
        Rule::parse("sub:0"),
        Err(GameError(
            "invalid take \"0\" in \"sub:0\" (expected 1 to 1000)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("sub:1001"),
        Err(GameError(
            "invalid take \"1001\" in \"sub:1001\" (expected 1 to 1000)".to_owned()
        ))
    );
}

#[test]
fn invalid_rule_diagnostics_sanitize_control_and_unicode_text() {
    assert_eq!(
        Rule::parse("SUB:1\né"),
        Err(GameError(
            "invalid rule \"SUB:1??\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("Oct:77"),
        Err(GameError(
            "invalid rule \"Oct:77\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
}

#[test]
fn malformed_octal_syntax_precedes_the_digit_count() {
    assert_eq!(
        Rule::parse("oct:8888888888888"),
        Err(GameError(
            "invalid rule \"oct:8888888888888\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("oct:"),
        Err(GameError(
            "invalid rule \"oct:\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
}

#[test]
fn octal_digit_count_precedes_the_final_zero_check() {
    // The digit count error wins when the rule also has a final zero.
    assert_eq!(
        Rule::parse("oct:7777777777770"),
        Err(GameError(
            "too many digits in \"oct:7777777777770\" (at most 12)".to_owned()
        ))
    );
}

#[test]
fn octal_rules_reject_digits_above_seven_and_final_zero() {
    assert_eq!(
        Rule::parse("oct:8"),
        Err(GameError(
            "invalid rule \"oct:8\" (expected nim, sub:1,3,4 or oct:77)".to_owned()
        ))
    );
    assert_eq!(
        Rule::parse("oct:70"),
        Err(GameError(
            "the last digit of \"oct:70\" must not be 0".to_owned()
        ))
    );
}

#[test]
fn nim_moves_are_empty_at_zero_and_keep_positive_remainders() {
    assert_eq!(moves::of(&Rule::Nim, 0), Vec::<Move>::new());
    assert_eq!(
        moves::of(&Rule::Nim, 3),
        vec![
            Move {
                take: 1,
                leaves: vec![2]
            },
            Move {
                take: 2,
                leaves: vec![1]
            },
            Move {
                take: 3,
                leaves: vec![]
            }
        ]
    );
}

#[test]
fn a_single_token_nim_heap_has_only_the_take_all_move() {
    assert_eq!(
        moves::of(&Rule::Nim, 1),
        vec![Move {
            take: 1,
            leaves: vec![]
        }]
    );
}

#[test]
fn subtraction_moves_skip_takes_above_the_heap_and_omit_zero_remainders() {
    let rule = Rule::Sub(vec![1, 3, 4]);
    assert_eq!(
        moves::of(&rule, 3),
        vec![
            Move {
                take: 1,
                leaves: vec![2]
            },
            Move {
                take: 3,
                leaves: vec![]
            }
        ]
    );
    assert_eq!(moves::of(&rule, 0), Vec::<Move>::new());
    assert_eq!(
        moves::of(&rule, 1),
        vec![Move {
            take: 1,
            leaves: vec![]
        }]
    );
}

#[test]
fn octal_zero_heap_and_take_beyond_the_heap_have_no_moves() {
    assert_eq!(moves::of(&Rule::Oct(vec![7, 7]), 0), Vec::<Move>::new());
    assert_eq!(moves::of(&Rule::Oct(vec![0, 7]), 1), Vec::<Move>::new());
}

#[test]
fn octal_bit_one_allows_only_a_no_heap_remainder() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![1]), 1),
        vec![Move {
            take: 1,
            leaves: vec![]
        }]
    );
    assert_eq!(moves::of(&Rule::Oct(vec![1]), 2), Vec::<Move>::new());
}

#[test]
fn octal_bit_two_allows_one_positive_remainder_heap() {
    assert_eq!(moves::of(&Rule::Oct(vec![2]), 1), Vec::<Move>::new());
    assert_eq!(
        moves::of(&Rule::Oct(vec![2]), 3),
        vec![Move {
            take: 1,
            leaves: vec![2]
        }]
    );
}

#[test]
fn octal_bits_one_and_two_combine_no_heap_and_one_heap_permissions() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![3]), 1),
        vec![Move {
            take: 1,
            leaves: vec![]
        }]
    );
    assert_eq!(
        moves::of(&Rule::Oct(vec![3]), 3),
        vec![Move {
            take: 1,
            leaves: vec![2]
        }]
    );
}

#[test]
fn octal_bits_one_and_four_combine_no_heap_and_split_permissions() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![5]), 1),
        vec![Move {
            take: 1,
            leaves: vec![]
        }]
    );
    assert_eq!(
        moves::of(&Rule::Oct(vec![5]), 4),
        vec![Move {
            take: 1,
            leaves: vec![1, 2]
        }]
    );
}

#[test]
fn octal_bits_two_and_four_order_one_heap_before_split_moves() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![6]), 3),
        vec![
            Move {
                take: 1,
                leaves: vec![2]
            },
            Move {
                take: 1,
                leaves: vec![1, 1]
            }
        ]
    );
}

#[test]
fn octal_digit_seven_orders_no_heap_one_heap_then_splits() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![7]), 1),
        vec![Move {
            take: 1,
            leaves: vec![]
        }]
    );
    assert_eq!(
        moves::of(&Rule::Oct(vec![7]), 3),
        vec![
            Move {
                take: 1,
                leaves: vec![2]
            },
            Move {
                take: 1,
                leaves: vec![1, 1]
            }
        ]
    );
}

#[test]
fn octal_seventy_seven_size_five_has_the_exact_ordered_move_list() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![7, 7]), 5),
        vec![
            Move {
                take: 1,
                leaves: vec![4]
            },
            Move {
                take: 1,
                leaves: vec![1, 3]
            },
            Move {
                take: 1,
                leaves: vec![2, 2]
            },
            Move {
                take: 2,
                leaves: vec![3]
            },
            Move {
                take: 2,
                leaves: vec![1, 2]
            }
        ]
    );
}

#[test]
fn octal_split_order_includes_equal_halves_once() {
    assert_eq!(
        moves::of(&Rule::Oct(vec![4]), 5),
        vec![
            Move {
                take: 1,
                leaves: vec![1, 3]
            },
            Move {
                take: 1,
                leaves: vec![2, 2]
            }
        ]
    );
}

#[test]
fn octal_zero_digit_allows_nothing_and_later_digit_uses_its_take_number() {
    assert_eq!(moves::of(&Rule::Oct(vec![0, 7]), 1), Vec::<Move>::new());
    assert_eq!(
        moves::of(&Rule::Oct(vec![0, 7]), 3),
        vec![Move {
            take: 2,
            leaves: vec![1]
        }]
    );
    assert_eq!(
        moves::of(&Rule::Oct(vec![0, 7]), 2),
        vec![Move {
            take: 2,
            leaves: vec![]
        }]
    );
}

#[test]
fn move_is_cloneable_debuggable_and_comparable_with_public_fields() {
    let movement = Move {
        take: 2,
        leaves: vec![1, 3],
    };
    assert_eq!(
        movement.clone(),
        Move {
            take: 2,
            leaves: vec![1, 3]
        }
    );
    assert_ne!(
        movement,
        Move {
            take: 2,
            leaves: vec![2, 2]
        }
    );
    assert_eq!(format!("{movement:?}"), "Move { take: 2, leaves: [1, 3] }");
}

#[test]
fn part_without_a_slash_has_the_exact_diagnostic() {
    assert_eq!(
        Part::parse("nim"),
        Err(GameError(
            "invalid part \"nim\" (expected RULE/SIZE such as nim/5 or oct:77/9)".to_owned()
        ))
    );
}

#[test]
fn part_splits_only_at_the_first_slash_and_checks_the_size_text() {
    assert_eq!(
        Part::parse("nim/3/4"),
        Err(GameError(
            "invalid size \"3/4\" (expected an integer from 0 to 2000)".to_owned()
        ))
    );
}

#[test]
fn part_reports_rule_errors_before_invalid_sizes() {
    assert_eq!(
        Part::parse("sub:0/2001"),
        Err(GameError(
            "invalid take \"0\" in \"sub:0\" (expected 1 to 1000)".to_owned()
        ))
    );
}

#[test]
fn part_accepts_zero_and_two_thousand_inclusive_size_bounds() {
    assert_eq!(
        Part::parse("nim/0"),
        Ok(Part {
            rule: Rule::Nim,
            size: 0
        })
    );
    assert_eq!(
        Part::parse("oct:77/2000"),
        Ok(Part {
            rule: Rule::Oct(vec![7, 7]),
            size: 2000
        })
    );
}

#[test]
fn part_names_use_canonical_rule_text() {
    assert_eq!(
        Part::parse("sub:4,1,3,3/12").expect("valid part").name(),
        "sub:1,3,4/12"
    );
    assert_eq!(
        Part::parse("oct:07/9").expect("valid part").name(),
        "oct:07/9"
    );
}

#[test]
fn part_rejects_sizes_above_the_bound_and_noncanonical_integer_text() {
    // Exact size-error cases: nim/2001, nim/01, nim/.
    assert_eq!(
        Part::parse("nim/2001"),
        Err(GameError(
            "invalid size \"2001\" (expected an integer from 0 to 2000)".to_owned()
        ))
    );
    assert_eq!(
        Part::parse("nim/01"),
        Err(GameError(
            "invalid size \"01\" (expected an integer from 0 to 2000)".to_owned()
        ))
    );
    assert_eq!(
        Part::parse("nim/"),
        Err(GameError(
            "invalid size \"\" (expected an integer from 0 to 2000)".to_owned()
        ))
    );
}

#[test]
fn part_fields_and_derived_traits_match_the_public_contract() {
    let part = Part {
        rule: Rule::Nim,
        size: 5,
    };
    assert_eq!(
        part.clone(),
        Part {
            rule: Rule::Nim,
            size: 5
        }
    );
    assert_ne!(
        part,
        Part {
            rule: Rule::Nim,
            size: 6
        }
    );
    assert_eq!(format!("{part:?}"), "Part { rule: Nim, size: 5 }");
}

#[test]
fn position_show_formats_empty_and_single_part_positions_without_newlines() {
    assert_eq!(position::show(&[]), "(empty)");
    assert_eq!(
        position::show(&[Part {
            rule: Rule::Nim,
            size: 5
        }]),
        "nim/5"
    );
}

#[test]
fn position_show_preserves_mixed_rule_and_zero_size_part_order() {
    assert_eq!(
        position::show(&[
            Part {
                rule: Rule::Oct(vec![7, 7]),
                size: 0
            },
            Part {
                rule: Rule::Nim,
                size: 3
            },
            Part {
                rule: Rule::Sub(vec![1, 3, 4]),
                size: 12
            }
        ]),
        "oct:77/0 nim/3 sub:1,3,4/12"
    );
}

#[test]
fn leaves_formats_empty_single_and_multiple_heaps_without_newlines() {
    assert_eq!(position::leaves(&[]), "none");
    assert_eq!(position::leaves(&[4]), "4");
    assert_eq!(position::leaves(&[2, 5]), "2+5");
}
