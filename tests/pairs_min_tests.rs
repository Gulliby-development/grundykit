use grundykit::cli;
use grundykit::cmd_pairs;
use grundykit::error::GameError;
use grundykit::io::Io;
use std::cell::{Cell, RefCell};

#[derive(Default)]
struct FakeIo {
    reads: Cell<usize>,
    stdout: RefCell<Vec<String>>,
    stderr: RefCell<Vec<String>>,
}

impl Io for FakeIo {
    fn read_stdin(&self) -> Vec<u8> {
        self.reads.set(self.reads.get() + 1);
        Vec::new()
    }

    fn out(&self, text: &str) {
        self.stdout.borrow_mut().push(text.to_owned());
    }

    fn err(&self, text: &str) {
        self.stderr.borrow_mut().push(text.to_owned());
    }
}

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn expect_output(argv: &[&str], expected: &str) {
    let io = FakeIo::default();
    let result = cmd_pairs::run(&args(argv), &io);
    assert_eq!(result, Ok(0));
    assert_eq!(&*io.stdout.borrow(), &[expected.to_owned()]);
    assert!(io.stderr.borrow().is_empty());
    assert_eq!(io.reads.get(), 0);
}

fn expect_error(argv: &[&str], message: &str) {
    let io = FakeIo::default();
    let result = cmd_pairs::run(&args(argv), &io);
    assert_eq!(result, Err(GameError(message.to_owned())));
    assert!(io.stdout.borrow().is_empty());
    assert!(io.stderr.borrow().is_empty());
}

#[test]
fn minimum_three_with_maximum_ten_lists_five_pairs() {
    expect_output(
        &["--min", "3", "--max", "10"],
        "min         3\nmax         10\ncount       5\npair        3 5\npair        4 7\npair        6 10\npair        8 13\npair        9 15\n",
    );
}

#[test]
fn option_order_does_not_matter() {
    expect_output(
        &["--max", "10", "--min", "3"],
        "min         3\nmax         10\ncount       5\npair        3 5\npair        4 7\npair        6 10\npair        8 13\npair        9 15\n",
    );
}

#[test]
fn inline_values_are_accepted_for_both_options() {
    expect_output(
        &["--min=10", "--max=20"],
        "min         10\nmax         20\ncount       6\npair        11 18\npair        12 20\npair        14 23\npair        16 26\npair        17 28\npair        19 31\n",
    );
}

#[test]
fn minimum_zero_prints_no_min_line() {
    expect_output(
        &["--min", "0", "--max", "10"],
        "max         10\ncount       7\npair        0 0\npair        1 2\npair        3 5\npair        4 7\npair        6 10\npair        8 13\npair        9 15\n",
    );
}

#[test]
fn explicit_zero_minimum_matches_the_default_output() {
    let plain = FakeIo::default();
    let explicit = FakeIo::default();
    assert_eq!(cmd_pairs::run(&args(&["--max=10"]), &plain), Ok(0));
    assert_eq!(
        cmd_pairs::run(&args(&["--min=0", "--max=10"]), &explicit),
        Ok(0)
    );
    assert_eq!(&*plain.stdout.borrow(), &*explicit.stdout.borrow());
}

#[test]
fn minimum_one_with_maximum_one_lists_one_pair() {
    expect_output(
        &["--min", "1", "--max", "1"],
        "min         1\nmax         1\ncount       1\npair        1 2\n",
    );
}

#[test]
fn minimum_that_is_not_a_smaller_pile_lists_no_pair() {
    expect_output(
        &["--min", "5", "--max", "5"],
        "min         5\nmax         5\ncount       0\n",
    );
}

#[test]
fn minimum_and_maximum_at_the_upper_bound_list_the_last_pair() {
    expect_output(
        &["--min", "20000", "--max", "20000"],
        "min         20000\nmax         20000\ncount       1\npair        20000 32361\n",
    );
}

#[test]
fn minimum_above_the_maximum_is_an_error() {
    expect_error(
        &["--min", "11", "--max", "10"],
        "--min 11 is larger than --max 10",
    );
}

#[test]
fn minimum_above_the_default_maximum_is_an_error() {
    expect_error(&["--min", "21"], "--min 21 is larger than --max 20");
}

#[test]
fn minimum_equal_to_the_default_maximum_is_fine() {
    expect_output(
        &["--min", "20"],
        "min         20\nmax         20\ncount       0\n",
    );
}

#[test]
fn minimum_out_of_range_is_an_error() {
    expect_error(
        &["--min=20001"],
        "invalid --min \"20001\" (expected an integer from 0 to 20000)",
    );
}

#[test]
fn minimum_with_a_leading_zero_is_an_error() {
    expect_error(
        &["--min", "05"],
        "invalid --min \"05\" (expected an integer from 0 to 20000)",
    );
}

#[test]
fn minimum_without_a_value_is_an_error() {
    expect_error(&["--min"], "--min needs a value");
}

#[test]
fn positional_argument_is_still_rejected() {
    expect_error(&["--min", "3", "10"], "too many arguments");
}

#[test]
fn cli_dispatch_passes_the_minimum_through() {
    let io = FakeIo::default();
    let code = cli::run(&args(&["pairs", "--min", "9", "--max", "9"]), &io);
    assert_eq!(code, 0);
    assert_eq!(
        &*io.stdout.borrow(),
        &["min         9\nmax         9\ncount       1\npair        9 15\n".to_owned()]
    );
}

#[test]
fn cli_reports_the_minimum_error_with_the_program_prefix() {
    let io = FakeIo::default();
    let code = cli::run(&args(&["pairs", "--min", "30"]), &io);
    assert_eq!(code, 2);
    assert!(io.stdout.borrow().is_empty());
    assert_eq!(
        &*io.stderr.borrow(),
        &["grundykit: --min 30 is larger than --max 20\n".to_owned()]
    );
}
