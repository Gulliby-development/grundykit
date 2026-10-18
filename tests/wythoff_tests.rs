use grundykit::wythoff::{Where, pairs, winning};

#[test]
fn zero_limit_contains_only_the_origin() {
    assert_eq!(pairs(0), vec![(0, 0)]);
}

#[test]
fn first_pair_is_included_when_its_smaller_pile_is_at_the_limit() {
    assert_eq!(pairs(1), vec![(0, 0), (1, 2)]);
}

#[test]
fn maximum_ten_pairs_match_the_literal_sequence() {
    assert_eq!(
        pairs(10),
        vec![(0, 0), (1, 2), (3, 5), (4, 7), (6, 10), (8, 13), (9, 15),]
    );
}

#[test]
fn a_partner_above_the_limit_is_kept_when_its_smaller_pile_is_included() {
    assert_eq!(pairs(2), vec![(0, 0), (1, 2)]);
}

#[test]
fn the_permitted_maximum_includes_the_last_smaller_pile_pair() {
    let boundary_pairs = pairs(20000);
    assert_eq!(boundary_pairs.len(), 12362);
    assert_eq!(boundary_pairs[0], (0, 0));
    assert_eq!(boundary_pairs[1], (1, 2));
    assert_eq!(boundary_pairs[12361], (20000, 32361));
}

#[test]
fn where_variants_have_the_required_traits_and_debug_names() {
    let both = Where::Both;
    let copied = both;
    let cloned = both.clone();
    assert_eq!(both, copied);
    assert_eq!(both, cloned);
    assert_eq!(format!("{:?}", Where::Pile1), "Pile1");
    assert_eq!(format!("{:?}", Where::Pile2), "Pile2");
    assert_eq!(format!("{:?}", Where::Both), "Both");
}

#[test]
fn the_origin_is_cold_and_has_no_winning_move() {
    assert_eq!(winning(0, 0), None);
}

#[test]
fn the_first_nonzero_cold_pair_is_losing_in_both_orientations() {
    assert_eq!(winning(1, 2), None);
    assert_eq!(winning(2, 1), None);
}

#[test]
fn another_cold_pair_is_losing_in_both_orientations() {
    assert_eq!(winning(3, 5), None);
    assert_eq!(winning(5, 3), None);
}

#[test]
fn both_piles_move_to_the_cold_pair_three_five() {
    assert_eq!(winning(4, 6), Some((1, Where::Both, (3, 5))));
}

#[test]
fn pile_two_move_preserves_the_input_orientation() {
    assert_eq!(winning(5, 8), Some((5, Where::Pile2, (5, 3))));
}

#[test]
fn reversed_piles_use_a_pile_one_move_and_preserve_orientation() {
    assert_eq!(winning(8, 5), Some((5, Where::Pile1, (3, 5))));
}

#[test]
fn pile_one_precedes_a_tied_pile_two_move() {
    assert_eq!(winning(2, 2), Some((1, Where::Pile1, (1, 2))));
}

#[test]
fn equal_piles_at_the_permitted_boundary_can_both_be_taken_to_zero() {
    assert_eq!(winning(20000, 20000), Some((20000, Where::Both, (0, 0))));
}

#[test]
fn an_empty_first_pile_can_win_by_taking_the_second_to_zero() {
    assert_eq!(winning(0, 20000), Some((20000, Where::Pile2, (0, 0))));
}

#[test]
fn an_empty_second_pile_can_win_by_taking_the_first_to_zero() {
    assert_eq!(winning(20000, 0), Some((20000, Where::Pile1, (0, 0))));
}

#[test]
fn a_larger_legal_take_is_selected_when_it_is_the_cold_target() {
    assert_eq!(winning(5, 8), Some((5, Where::Pile2, (5, 3))));
}
