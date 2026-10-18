use grundykit::nimber::{inverse, product};

#[test]
fn nim_product_of_zero_is_zero_in_both_operand_orders() {
    assert_eq!(product(0, 0), 0);
    // Spec case: product(0,5) = 0.
    assert_eq!(product(0, 5), 0);
    assert_eq!(product(5, 0), 0);
}

#[test]
fn nim_product_of_one_is_the_other_operand_at_full_range() {
    assert_eq!(product(1, 1), 1);
    // Spec case: product(1,4294967295) = 4294967295.
    assert_eq!(product(1, 4294967295), 4294967295);
    assert_eq!(product(4294967295, 1), 4294967295);
}

#[test]
fn nim_product_of_two_by_two_is_three() {
    // Spec case: product(2,2) = 3.
    assert_eq!(product(2, 2), 3);
}

#[test]
fn nim_product_of_four_by_four_is_six() {
    // Spec case: product(4,4) = 6.
    assert_eq!(product(4, 4), 6);
}

#[test]
fn nim_product_of_eight_by_eight_is_thirteen() {
    // Spec case: product(8,8) = 13.
    assert_eq!(product(8, 8), 13);
}

#[test]
fn nim_product_of_seven_by_nine_is_eight() {
    // Spec case: product(7,9) = 8.
    assert_eq!(product(7, 9), 8);
}

#[test]
fn reversed_literal_full_range_operands_have_the_expected_products() {
    assert_eq!(product(9, 7), 8);
    // Spec case: product(4294967295,12255232) = 1.
    assert_eq!(product(12255232, 4294967295), 1);
}

#[test]
fn nim_product_of_the_largest_u32_nimber_with_itself_is_literal() {
    // Spec case: product(4294967295,4294967295) = 2628753316.
    assert_eq!(product(4294967295, 4294967295), 2628753316);
}

#[test]
fn zero_has_no_nim_multiplicative_inverse() {
    assert_eq!(inverse(0), None);
}

#[test]
fn one_is_its_own_nim_multiplicative_inverse() {
    assert_eq!(inverse(1), Some(1));
}

#[test]
fn specified_small_nimber_inverses_are_literal() {
    assert_eq!(inverse(7), Some(11));
    assert_eq!(inverse(9), Some(6));
    assert_eq!(inverse(5), Some(12));
}

#[test]
fn largest_u32_nimber_has_the_specified_inverse() {
    assert_eq!(inverse(4294967295), Some(12255232));
}

#[test]
fn largest_nimber_times_its_inverse_is_one() {
    assert_eq!(product(4294967295, 12255232), 1);
}
