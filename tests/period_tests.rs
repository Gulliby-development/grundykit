use grundykit::period::{Period, detect, detect_with};

#[test]
fn empty_sequence_has_no_period() {
    assert_eq!(detect(&[]), None);
}

#[test]
fn one_or_two_values_do_not_contain_three_complete_periods() {
    assert_eq!(detect(&[7]), None);
    assert_eq!(detect(&[7, 7]), None);
}

#[test]
fn exactly_three_equal_values_support_period_one_from_the_start() {
    assert_eq!(detect(&[7, 7, 7]), Some(Period { pre: 0, period: 1 }));
}

#[test]
fn exactly_three_alternating_cycles_support_period_two() {
    assert_eq!(
        detect(&[0, 1, 0, 1, 0, 1]),
        Some(Period { pre: 0, period: 2 })
    );
}

#[test]
fn a_sequence_one_value_short_of_three_cycles_has_no_period() {
    assert_eq!(detect(&[0, 1, 0, 1, 0]), None);
}

#[test]
fn period_can_begin_after_a_prefix_at_the_three_cycle_threshold() {
    assert_eq!(
        detect(&[9, 0, 1, 0, 1, 0, 1]),
        Some(Period { pre: 1, period: 2 })
    );
}

#[test]
fn a_trailing_mismatch_prevents_an_otherwise_repeating_suffix() {
    assert_eq!(detect(&[0, 1, 0, 1, 0, 1, 9]), None);
}

#[test]
fn smallest_period_is_selected_even_when_its_preperiod_is_late() {
    assert_eq!(
        detect(&[0, 1, 1, 1, 0, 1, 1, 1, 0, 1, 1, 1]),
        Some(Period { pre: 9, period: 1 })
    );
}

#[test]
fn the_earliest_valid_preperiod_is_selected_for_a_period() {
    assert_eq!(detect(&[9, 7, 7, 7, 7]), Some(Period { pre: 1, period: 1 }));
}

#[test]
fn period_two_is_selected_over_larger_valid_periods() {
    assert_eq!(
        detect(&[0, 1, 0, 1, 0, 1, 0, 1]),
        Some(Period { pre: 0, period: 2 })
    );
}

#[test]
fn extreme_u64_values_are_compared_without_numeric_restrictions() {
    assert_eq!(
        detect(&[u64::MAX, 0, u64::MAX, 0, u64::MAX, 0]),
        Some(Period { pre: 0, period: 2 })
    );
}

#[test]
fn extreme_u64_values_do_not_match_unless_they_are_equal() {
    assert_eq!(detect(&[u64::MAX, 0, u64::MAX, 1, u64::MAX, 0]), None);
}

#[test]
fn two_cycles_are_enough_when_requested() {
    assert_eq!(
        detect_with(&[0, 1, 0, 1], 2),
        Some(Period { pre: 0, period: 2 })
    );
    assert_eq!(detect_with(&[0, 1, 0, 1], 3), None);
}

#[test]
fn more_cycles_can_reject_a_shorter_period() {
    let values = [0, 0, 1, 1, 2, 0, 0, 1, 1, 2];
    assert_eq!(detect_with(&values, 2), Some(Period { pre: 0, period: 5 }));
    assert_eq!(detect_with(&values, 3), None);
}

#[test]
fn three_cycles_stay_the_default() {
    let values = [9, 0, 1, 0, 1, 0, 1];
    assert_eq!(detect(&values), detect_with(&values, 3));
    assert_eq!(detect(&values), Some(Period { pre: 1, period: 2 }));
}

#[test]
fn period_exposes_the_public_fields_and_required_derives() {
    let period = Period { pre: 2, period: 3 };
    assert_eq!(period.clone(), period);
    assert_ne!(period, Period { pre: 3, period: 3 });
    assert_eq!(format!("{period:?}"), "Period { pre: 2, period: 3 }");
}
