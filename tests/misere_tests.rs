use grundykit::misere::{previous_wins, winning};

#[test]
fn empty_heap_list_is_a_next_player_win_without_a_move() {
    assert!(!previous_wins(&[]));
    assert_eq!(winning(&[]), None);
}

#[test]
fn all_zero_heaps_are_a_next_player_win_without_a_move() {
    assert!(!previous_wins(&[0, 0]));
    assert_eq!(winning(&[0, 0]), None);
}

#[test]
fn a_singleton_heap_is_losing_for_the_next_player() {
    assert!(previous_wins(&[1]));
    assert_eq!(winning(&[1]), None);
}

#[test]
fn an_odd_number_of_singletons_is_losing_for_the_next_player() {
    assert!(previous_wins(&[1, 1, 1]));
    assert_eq!(winning(&[1, 1, 1]), None);
}

#[test]
fn an_even_number_of_singletons_can_be_reduced_to_a_losing_position() {
    assert!(!previous_wins(&[1, 1]));
    assert_eq!(winning(&[1, 1]), Some((1, 1)));
}

#[test]
fn zero_and_two_singletons_choose_the_first_singleton_heap() {
    assert!(!previous_wins(&[0, 1, 1]));
    assert_eq!(winning(&[0, 1, 1]), Some((2, 1)));
}

#[test]
fn equal_big_heaps_with_zero_nim_sum_are_losing() {
    assert!(previous_wins(&[2, 2]));
    assert_eq!(winning(&[2, 2]), None);
}

#[test]
fn eight_equal_big_heaps_with_zero_nim_sum_are_losing() {
    assert!(previous_wins(&[2, 2, 2, 2, 2, 2, 2, 2]));
    assert_eq!(winning(&[2, 2, 2, 2, 2, 2, 2, 2]), None);
}

#[test]
fn one_big_heap_can_be_reduced_to_a_singleton_losing_position() {
    assert!(!previous_wins(&[2]));
    assert_eq!(winning(&[2]), Some((1, 1)));
}

#[test]
fn a_big_heap_with_a_singleton_can_be_taken_to_zero() {
    assert!(!previous_wins(&[2, 1]));
    assert_eq!(winning(&[2, 1]), Some((1, 2)));
}

#[test]
fn mixed_big_heaps_use_the_first_nim_zeroing_move() {
    assert!(!previous_wins(&[3, 4, 5]));
    assert_eq!(winning(&[3, 4, 5]), Some((1, 2)));
}

#[test]
fn zeros_mixed_with_big_heaps_keep_heap_order_across_eight_heaps() {
    assert!(!previous_wins(&[0, 2, 0, 3, 0, 0, 0, 0]));
    assert_eq!(winning(&[0, 2, 0, 3, 0, 0, 0, 0]), Some((4, 1)));
}

#[test]
fn the_heap_at_the_permitted_maximum_has_a_literal_winning_take() {
    assert!(!previous_wins(&[1000]));
    assert_eq!(winning(&[1000]), Some((1, 999)));
}

#[test]
fn the_first_heap_wins_ties_between_equal_singleton_moves() {
    assert_eq!(winning(&[1, 1]), Some((1, 1)));
}

#[test]
fn the_first_take_that_preserves_a_losing_position_is_returned() {
    assert_eq!(winning(&[3, 4, 5]), Some((1, 2)));
}
