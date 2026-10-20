use grundykit::cmd_grundy;
use grundykit::cmd_misere;
use grundykit::cmd_moves;
use grundykit::cmd_nimber;
use grundykit::cmd_pairs;
use grundykit::cmd_period;
use grundykit::cmd_play;
use grundykit::cmd_sum;
use grundykit::cmd_win;
use grundykit::cmd_wythoff;
use grundykit::error::GameError;
use grundykit::io::Io;
use std::cell::{Cell, RefCell};

#[derive(Default)]
struct FakeIo {
    stdin: Vec<u8>,
    reads: Cell<usize>,
    stdout: RefCell<Vec<String>>,
    stderr: RefCell<Vec<String>>,
}

impl FakeIo {
    fn with_stdin(stdin: Vec<u8>) -> Self {
        Self {
            stdin,
            ..Self::default()
        }
    }
}

impl Io for FakeIo {
    fn read_stdin(&self) -> Vec<u8> {
        self.reads.set(self.reads.get() + 1);
        self.stdin.clone()
    }

    fn out(&self, text: &str) {
        self.stdout.borrow_mut().push(text.to_owned());
    }

    fn err(&self, text: &str) {
        self.stderr.borrow_mut().push(text.to_owned());
    }
}

macro_rules! args {
    ($($value:expr),* $(,)?) => {
        vec![$(String::from($value)),*]
    };
}

fn assert_success(result: Result<i32, GameError>, code: i32, io: &FakeIo, expected: &str) {
    assert_eq!(result, Ok(code));
    assert_eq!(&*io.stdout.borrow(), &[expected.to_owned()]);
    assert!(io.stderr.borrow().is_empty());
    assert!(expected.ends_with('\n'));
    assert!(!expected.contains('\r'));
}

fn assert_error(result: Result<i32, GameError>, message: &str, io: &FakeIo, reads: usize) {
    assert_eq!(result, Err(GameError(message.to_owned())));
    assert!(io.stdout.borrow().is_empty());
    assert!(io.stderr.borrow().is_empty());
    assert_eq!(io.reads.get(), reads);
}

#[test]
fn grundy_prints_the_supplied_kayles_example_exactly() {
    let io = FakeIo::default();
    let result = cmd_grundy::run(&args!["oct:77", "--max", "19"], &io);

    assert_success(
        result,
        0,
        &io,
        "rule        oct:77\nmax         19\nvalues      0 1 2 3 1 4 3 2 1 4 2 6 4 1 2 7 1 4 3 2\nzeros       0\nlargest     7\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn grundy_uses_default_max_and_accepts_zero_as_the_lower_bound() {
    let default_io = FakeIo::default();
    let default_result = cmd_grundy::run(&args!["nim"], &default_io);
    assert_eq!(default_result, Ok(0));
    assert_eq!(default_io.stdout.borrow().len(), 1);
    assert!(default_io.stderr.borrow().is_empty());
    let default_output = default_io.stdout.borrow();
    let default_lines = default_output[0].lines().collect::<Vec<_>>();
    assert_eq!(default_lines.len(), 5);
    assert_eq!(default_lines[0], "rule        nim");
    assert_eq!(default_lines[1], "max         20");
    assert_eq!(default_lines[3], "zeros       0");
    assert_eq!(default_lines[4], "largest     20");
    let default_values = default_lines[2].split_whitespace().collect::<Vec<_>>();
    assert_eq!(default_values.len(), 22);
    assert_eq!(default_values[0], "values");
    assert_eq!(default_values[1], "0");
    assert_eq!(default_values[2], "1");
    assert_eq!(default_values[11], "10");
    assert_eq!(default_values[21], "20");
    assert!(default_output[0].ends_with('\n'));
    assert!(!default_output[0].contains('\r'));

    let zero_io = FakeIo::default();
    let zero_result = cmd_grundy::run(&args!["--max=0", "nim"], &zero_io);
    assert_success(
        zero_result,
        0,
        &zero_io,
        "rule        nim\nmax         0\nvalues      0\nzeros       0\nlargest     0\n",
    );
}

#[test]
fn grundy_accepts_inline_separate_and_last_repeated_max_values() {
    let inline_io = FakeIo::default();
    let inline_result = cmd_grundy::run(&args!["--max=8", "nim"], &inline_io);
    assert_success(
        inline_result,
        0,
        &inline_io,
        "rule        nim\nmax         8\nvalues      0 1 2 3 4 5 6 7 8\nzeros       0\nlargest     8\n",
    );

    let repeated_io = FakeIo::default();
    let repeated_result = cmd_grundy::run(&args!["--max=bad", "nim", "--max", "0"], &repeated_io);
    assert_success(
        repeated_result,
        0,
        &repeated_io,
        "rule        nim\nmax         0\nvalues      0\nzeros       0\nlargest     0\n",
    );
}

#[test]
fn grundy_rejects_missing_excess_arguments_and_options_before_values() {
    let missing_io = FakeIo::default();
    let missing = cmd_grundy::run(&args![], &missing_io);
    assert_error(missing, "missing RULE", &missing_io, 0);

    let excess_io = FakeIo::default();
    let excess = cmd_grundy::run(&args!["nim", "oct:77"], &excess_io);
    assert_error(excess, "too many arguments", &excess_io, 0);

    let help_io = FakeIo::default();
    let help = cmd_grundy::run(&args!["--help"], &help_io);
    assert_error(help, "unknown option \"--help\"", &help_io, 0);

    let dash_io = FakeIo::default();
    let dash = cmd_grundy::run(&args!["--"], &dash_io);
    assert_error(dash, "unknown option \"--\"", &dash_io, 0);

    let precedence_io = FakeIo::default();
    let precedence = cmd_grundy::run(&args!["--max=2001", "nim", "extra"], &precedence_io);
    assert_error(
        precedence,
        "invalid --max \"2001\" (expected an integer from 0 to 2000)",
        &precedence_io,
        0,
    );

    let missing_option_io = FakeIo::default();
    let missing_option = cmd_grundy::run(&args!["--max"], &missing_option_io);
    assert_error(missing_option, "--max needs a value", &missing_option_io, 0);
}

#[test]
fn moves_prints_the_supplied_kayles_moves_in_order() {
    let io = FakeIo::default();
    let result = cmd_moves::run(&args!["oct:77/5"], &io);

    assert_success(
        result,
        0,
        &io,
        "part        oct:77/5\nvalue       4\nmoves       5\nmove        take 1  leaves 4  value 1\nmove        take 1  leaves 1+3  value 2\nmove        take 1  leaves 2+2  value 0\nmove        take 2  leaves 3  value 3\nmove        take 2  leaves 1+2  value 3\n",
    );
}

#[test]
fn moves_canonicalize_subtraction_rules_and_keep_terminal_heap_output() {
    let sub_io = FakeIo::default();
    let sub_result = cmd_moves::run(&args!["sub:4,1,3,3/3"], &sub_io);
    assert_success(
        sub_result,
        0,
        &sub_io,
        "part        sub:1,3,4/3\nvalue       1\nmoves       2\nmove        take 1  leaves 2  value 0\nmove        take 3  leaves none  value 0\n",
    );

    let terminal_io = FakeIo::default();
    let terminal_result = cmd_moves::run(&args!["nim/0"], &terminal_io);
    assert_success(
        terminal_result,
        0,
        &terminal_io,
        "part        nim/0\nvalue       0\nmoves       0\n",
    );
}

#[test]
fn moves_rejects_arity_and_invalid_parts_without_reading_stdin() {
    let missing_io = FakeIo::default();
    let missing = cmd_moves::run(&args![], &missing_io);
    assert_error(missing, "missing PART", &missing_io, 0);

    let excess_io = FakeIo::default();
    let excess = cmd_moves::run(&args!["nim/1", "nim/2"], &excess_io);
    assert_error(excess, "too many arguments", &excess_io, 0);

    let part_io = FakeIo::default();
    let part = cmd_moves::run(&args!["nim"], &part_io);
    assert_error(
        part,
        "invalid part \"nim\" (expected RULE/SIZE such as nim/5 or oct:77/9)",
        &part_io,
        0,
    );

    let size_io = FakeIo::default();
    let size = cmd_moves::run(&args!["nim/3/4"], &size_io);
    assert_error(
        size,
        "invalid size \"3/4\" (expected an integer from 0 to 2000)",
        &size_io,
        0,
    );
}

#[test]
fn sum_prints_both_supplied_winner_examples() {
    let next_io = FakeIo::default();
    let next = cmd_sum::run(&args!["nim/3", "nim/4", "nim/5"], &next_io);
    assert_success(
        next,
        0,
        &next_io,
        "parts       3\npart        nim/3  value 3\npart        nim/4  value 4\npart        nim/5  value 5\nvalue       2\nwinner      next\n",
    );

    let previous_io = FakeIo::default();
    let previous = cmd_sum::run(&args!["nim/2", "nim/2"], &previous_io);
    assert_success(
        previous,
        1,
        &previous_io,
        "parts       2\npart        nim/2  value 2\npart        nim/2  value 2\nvalue       0\nwinner      previous\n",
    );
}

#[test]
fn sum_preserves_zero_parts_and_canonicalizes_mixed_rules() {
    let zero_io = FakeIo::default();
    let zero = cmd_sum::run(&args!["nim/0"], &zero_io);
    assert_success(
        zero,
        1,
        &zero_io,
        "parts       1\npart        nim/0  value 0\nvalue       0\nwinner      previous\n",
    );

    let mixed_io = FakeIo::default();
    let mixed = cmd_sum::run(&args!["sub:4,1,3,3/3", "nim/2", "nim/0"], &mixed_io);
    assert_success(
        mixed,
        0,
        &mixed_io,
        "parts       3\npart        sub:1,3,4/3  value 1\npart        nim/2  value 2\npart        nim/0  value 0\nvalue       3\nwinner      next\n",
    );
}

#[test]
fn win_prints_the_supplied_nim_and_mixed_rule_moves_exactly() {
    let nim_io = FakeIo::default();
    let nim = cmd_win::run(&args!["nim/3", "nim/4", "nim/5"], &nim_io);
    assert_success(
        nim,
        0,
        &nim_io,
        "value       2\npart        1\ntake        2\nleaves      1\nafter       nim/1 nim/4 nim/5\n",
    );

    let mixed_io = FakeIo::default();
    let mixed = cmd_win::run(&args!["oct:77/9", "nim/6", "sub:1,3,4/7"], &mixed_io);
    assert_success(
        mixed,
        0,
        &mixed_io,
        "value       2\npart        1\ntake        2\nleaves      2+5\nafter       oct:77/2 oct:77/5 nim/6 sub:1,3,4/7\n",
    );
}

#[test]
fn win_prints_no_winning_move_and_preserves_untouched_zero_parts() {
    let losing_io = FakeIo::default();
    let losing = cmd_win::run(&args!["nim/2", "nim/2"], &losing_io);
    assert_success(
        losing,
        1,
        &losing_io,
        "value       0\nstatus      no winning move\n",
    );

    let zero_io = FakeIo::default();
    let zero = cmd_win::run(&args!["nim/1", "nim/0"], &zero_io);
    assert_success(
        zero,
        0,
        &zero_io,
        "value       1\npart        1\ntake        1\nleaves      none\nafter       nim/0\n",
    );

    let empty_after_io = FakeIo::default();
    let empty_after = cmd_win::run(&args!["nim/1"], &empty_after_io);
    assert_success(
        empty_after,
        0,
        &empty_after_io,
        "value       1\npart        1\ntake        1\nleaves      none\nafter       (empty)\n",
    );
}

#[test]
fn multipart_commands_validate_arity_then_values_left_to_right() {
    let missing_sum_io = FakeIo::default();
    let missing_sum = cmd_sum::run(&args![], &missing_sum_io);
    assert_error(missing_sum, "missing PART", &missing_sum_io, 0);

    let nine_sum_io = FakeIo::default();
    let nine_sum = cmd_sum::run(
        &args![
            "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0"
        ],
        &nine_sum_io,
    );
    assert_error(nine_sum, "too many parts (at most 8)", &nine_sum_io, 0);

    let unknown_option_io = FakeIo::default();
    let unknown_option = cmd_sum::run(&args!["--help"], &unknown_option_io);
    assert_error(
        unknown_option,
        "unknown option \"--help\"",
        &unknown_option_io,
        0,
    );

    let bad_first_io = FakeIo::default();
    let bad_first = cmd_sum::run(&args!["nim/bad", "oct:70/not-a-size"], &bad_first_io);
    assert_error(
        bad_first,
        "invalid size \"bad\" (expected an integer from 0 to 2000)",
        &bad_first_io,
        0,
    );

    let play_size_io = FakeIo::default();
    let bad_later_before_total = cmd_play::run(&args!["nim/30", "nim/11/4"], &play_size_io);
    assert_error(
        bad_later_before_total,
        "invalid size \"11/4\" (expected an integer from 0 to 2000)",
        &play_size_io,
        0,
    );
}

#[test]
fn period_prints_both_supplied_success_examples_and_no_answer() {
    let oct_io = FakeIo::default();
    let oct = cmd_period::run(&args!["oct:77", "--max", "200"], &oct_io);
    assert_success(
        oct,
        0,
        &oct_io,
        "rule        oct:77\nmax         200\npreperiod   71\nperiod      12\ncycle       7 4 1 2 8 1 4 7 2 1 8 2\n",
    );

    let sub_io = FakeIo::default();
    let sub = cmd_period::run(&args!["sub:2,3", "--max=14"], &sub_io);
    assert_success(
        sub,
        0,
        &sub_io,
        "rule        sub:2,3\nmax         14\npreperiod   0\nperiod      5\ncycle       0 0 1 1 2\n",
    );

    let none_io = FakeIo::default();
    let none = cmd_period::run(&args!["sub:2,3", "--max", "13"], &none_io);
    assert_success(
        none,
        1,
        &none_io,
        "rule        sub:2,3\nmax         13\nperiod      none\n",
    );
}

#[test]
fn period_default_and_inclusive_minimum_are_checked() {
    let default_io = FakeIo::default();
    let default = cmd_period::run(&args!["nim"], &default_io);
    assert_success(
        default,
        1,
        &default_io,
        "rule        nim\nmax         300\nperiod      none\n",
    );

    let minimum_io = FakeIo::default();
    let minimum = cmd_period::run(&args!["nim", "--max=1"], &minimum_io);
    assert_success(
        minimum,
        1,
        &minimum_io,
        "rule        nim\nmax         1\nperiod      none\n",
    );

    let invalid_io = FakeIo::default();
    let invalid = cmd_period::run(&args!["--max=0", "nim"], &invalid_io);
    assert_error(
        invalid,
        "invalid --max \"0\" (expected an integer from 1 to 5000)",
        &invalid_io,
        0,
    );
}

#[test]
fn play_prints_the_supplied_six_turn_nim_transcript_verbatim() {
    let io = FakeIo::default();
    let result = cmd_play::run(&args!["nim/1", "nim/2", "nim/3"], &io);

    assert_success(
        result,
        0,
        &io,
        "start       nim/1 nim/2 nim/3\nmove        1  A  part 1  take 1  ->  nim/2 nim/3\nmove        2  B  part 2  take 1  ->  nim/2 nim/2\nmove        3  A  part 1  take 1  ->  nim/1 nim/2\nmove        4  B  part 2  take 1  ->  nim/1 nim/1\nmove        5  A  part 1  take 1  ->  nim/1\nmove        6  B  part 1  take 1  ->  (empty)\nmoves       6\nwinner      B\n",
    );
}

#[test]
fn play_prints_the_supplied_split_heap_transcript_verbatim() {
    let io = FakeIo::default();
    let result = cmd_play::run(&args!["oct:77/5", "nim/2"], &io);

    assert_success(
        result,
        0,
        &io,
        "start       oct:77/5 nim/2\nmove        1  A  part 1  take 1  ->  oct:77/1 oct:77/3 nim/2\nmove        2  B  part 1  take 1  ->  oct:77/3 nim/2\nmove        3  A  part 1  take 1  ->  oct:77/2 nim/2\nmove        4  B  part 1  take 1  ->  oct:77/1 nim/2\nmove        5  A  part 2  take 1  ->  oct:77/1 nim/1\nmove        6  B  part 1  take 1  ->  nim/1\nmove        7  A  part 1  take 1  ->  (empty)\nmoves       7\nwinner      A\n",
    );
}

#[test]
fn play_accepts_size_forty_and_reports_initially_terminal_positions() {
    let boundary_io = FakeIo::default();
    let boundary = cmd_play::run(&args!["nim/40"], &boundary_io);
    assert_success(
        boundary,
        0,
        &boundary_io,
        "start       nim/40\nmove        1  A  part 1  take 40  ->  (empty)\nmoves       1\nwinner      A\n",
    );

    let terminal_io = FakeIo::default();
    let terminal = cmd_play::run(&args!["nim/0"], &terminal_io);
    assert_success(
        terminal,
        0,
        &terminal_io,
        "start       nim/0\nmoves       0\nwinner      B\n",
    );
}

#[test]
fn play_rejects_size_forty_one_after_parsing_all_parts() {
    let io = FakeIo::default();
    let result = cmd_play::run(&args!["nim/30", "nim/11"], &io);
    assert_error(
        result,
        "total size 41 is above 40 (too large for play)",
        &io,
        0,
    );
}

#[test]
fn sum_reads_bom_and_all_four_ascii_whitespace_separators_once() {
    let input = "\u{feff}nim/1  nim/2\tnim/3\r\nnim/0";
    let io = FakeIo::with_stdin(input.as_bytes().to_vec());
    let result = cmd_sum::run(&args!["-"], &io);
    assert_success(
        result,
        1,
        &io,
        "parts       4\npart        nim/1  value 1\npart        nim/2  value 2\npart        nim/3  value 3\npart        nim/0  value 0\nvalue       0\nwinner      previous\n",
    );
    assert_eq!(io.reads.get(), 1);
}

#[test]
fn multipart_stdin_errors_are_exact_and_silent() {
    let empty_io = FakeIo::with_stdin(b" \t\r\n".to_vec());
    let empty = cmd_sum::run(&args!["-"], &empty_io);
    assert_error(empty, "<stdin>: no input", &empty_io, 1);

    let malformed_io = FakeIo::with_stdin(vec![0x6e, 0x69, 0x6d, 0x2f, 0x33, 0xff, 0x0a]);
    let malformed = cmd_win::run(&args!["-"], &malformed_io);
    assert_error(
        malformed,
        "<stdin>: input is not valid UTF-8",
        &malformed_io,
        1,
    );

    let too_many_io =
        FakeIo::with_stdin(b"nim/0 nim/0 nim/0 nim/0 nim/0 nim/0 nim/0 nim/0 nim/0".to_vec());
    let too_many = cmd_play::run(&args!["-"], &too_many_io);
    assert_error(too_many, "too many parts (at most 8)", &too_many_io, 1);
}

#[test]
fn stdin_options_and_nonsole_dashes_are_ordinary_part_values() {
    let option_word_io = FakeIo::with_stdin(b"--max".to_vec());
    let option_word = cmd_sum::run(&args!["-"], &option_word_io);
    assert_error(
        option_word,
        "invalid part \"--max\" (expected RULE/SIZE such as nim/5 or oct:77/9)",
        &option_word_io,
        1,
    );

    let dash_io = FakeIo::default();
    let dash = cmd_sum::run(&args!["nim/1", "-"], &dash_io);
    assert_error(
        dash,
        "invalid part \"-\" (expected RULE/SIZE such as nim/5 or oct:77/9)",
        &dash_io,
        0,
    );

    let option_before_stdin_io = FakeIo::with_stdin(b"nim/1".to_vec());
    let option_before_stdin = cmd_sum::run(&args!["--max", "-"], &option_before_stdin_io);
    assert_error(
        option_before_stdin,
        "unknown option \"--max\"",
        &option_before_stdin_io,
        0,
    );
}

#[test]
fn only_multipart_commands_expand_a_sole_dash() {
    // Each row is a literal argv vector after the command name. Only sum, win, and play expand
    // exactly ["-"] as stdin; grundy, moves, and period keep positional values and never read it.
    type Run = fn(&[String], &dyn Io) -> Result<i32, GameError>;
    let non_multipart_cases: [(Run, Vec<String>, &str, i32); 3] = [
        (
            cmd_grundy::run,
            args!["nim", "--max=19"],
            "rule        nim\nmax         19\nvalues      0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19\nzeros       0\nlargest     19\n",
            0,
        ),
        (
            cmd_moves::run,
            args!["nim/1"],
            "part        nim/1\nvalue       1\nmoves       1\nmove        take 1  leaves none  value 0\n",
            0,
        ),
        (
            cmd_period::run,
            args!["nim", "--max", "1"],
            "rule        nim\nmax         1\nperiod      none\n",
            1,
        ),
    ];

    for (run, argv, expected, code) in non_multipart_cases {
        let io = FakeIo::with_stdin(b"unused".to_vec());
        let result = run(&argv, &io);
        assert_success(result, code, &io, expected);
        assert_eq!(io.reads.get(), 0);
    }

    let sum_io = FakeIo::with_stdin(b"nim/1".to_vec());
    let sum = cmd_sum::run(&args!["-"], &sum_io);
    assert_success(
        sum,
        0,
        &sum_io,
        "parts       1\npart        nim/1  value 1\nvalue       1\nwinner      next\n",
    );
    assert_eq!(sum_io.reads.get(), 1);
}

#[test]
fn win_and_play_expand_stdin_once() {
    let win_io = FakeIo::with_stdin(b"nim/1 nim/0".to_vec());
    let win = cmd_win::run(&args!["-"], &win_io);
    assert_success(
        win,
        0,
        &win_io,
        "value       1\npart        1\ntake        1\nleaves      none\nafter       nim/0\n",
    );
    assert_eq!(win_io.reads.get(), 1);

    let play_io = FakeIo::with_stdin(b"nim/1".to_vec());
    let play = cmd_play::run(&args!["-"], &play_io);
    assert_success(
        play,
        0,
        &play_io,
        "start       nim/1\nmove        1  A  part 1  take 1  ->  (empty)\nmoves       1\nwinner      A\n",
    );
    assert_eq!(play_io.reads.get(), 1);
}

#[test]
fn grundy_maximum_run_checks_count_and_selected_literal_entries() {
    let io = FakeIo::default();
    let result = cmd_grundy::run(&args!["nim", "--max=2000"], &io);

    assert_eq!(result, Ok(0));
    assert_eq!(io.stdout.borrow().len(), 1);
    assert!(io.stderr.borrow().is_empty());
    let output = io.stdout.borrow();
    assert!(output[0].starts_with("rule        nim\nmax         2000\n"));
    assert!(output[0].ends_with("zeros       0\nlargest     2000\n"));
    let value_fields = output[0]
        .lines()
        .nth(2)
        .expect("values line")
        .split_whitespace()
        .collect::<Vec<_>>();
    assert_eq!(value_fields.len(), 2002);
    assert_eq!(value_fields[0], "values");
    assert_eq!(value_fields[1], "0");
    assert_eq!(value_fields[2], "1");
    assert_eq!(value_fields[2001], "2000");
    assert!(!output[0].contains('\r'));
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn rule_parser_take_and_octal_diagnostics_are_forwarded_exactly() {
    let take_io = FakeIo::default();
    let take = cmd_grundy::run(&args!["sub:1,1001"], &take_io);
    assert_error(
        take,
        "invalid take \"1001\" in \"sub:1,1001\" (expected 1 to 1000)",
        &take_io,
        0,
    );

    let oct_io = FakeIo::default();
    let oct = cmd_grundy::run(&args!["oct:70"], &oct_io);
    assert_error(
        oct,
        "the last digit of \"oct:70\" must not be 0",
        &oct_io,
        0,
    );
}

#[test]
fn period_reports_missing_excess_and_noncanonical_option_values() {
    let missing_io = FakeIo::default();
    let missing = cmd_period::run(&args![], &missing_io);
    assert_error(missing, "missing RULE", &missing_io, 0);

    let excess_io = FakeIo::default();
    let excess = cmd_period::run(&args!["nim", "sub:1"], &excess_io);
    assert_error(excess, "too many arguments", &excess_io, 0);

    let leading_zero_io = FakeIo::default();
    let leading_zero = cmd_period::run(&args!["--max=01", "nim"], &leading_zero_io);
    assert_error(
        leading_zero,
        "invalid --max \"01\" (expected an integer from 1 to 5000)",
        &leading_zero_io,
        0,
    );

    let signed_io = FakeIo::default();
    let signed = cmd_grundy::run(&args!["--max=+3", "nim"], &signed_io);
    assert_error(
        signed,
        "invalid --max \"+3\" (expected an integer from 0 to 2000)",
        &signed_io,
        0,
    );

    let fraction_io = FakeIo::default();
    let fraction = cmd_grundy::run(&args!["--max=1.5", "nim"], &fraction_io);
    assert_error(
        fraction,
        "invalid --max \"1.5\" (expected an integer from 0 to 2000)",
        &fraction_io,
        0,
    );
}

#[test]
fn win_and_play_reject_missing_parts_and_nine_parts_before_output() {
    let missing_win_io = FakeIo::default();
    let missing_win = cmd_win::run(&args![], &missing_win_io);
    assert_error(missing_win, "missing PART", &missing_win_io, 0);

    let missing_play_io = FakeIo::default();
    let missing_play = cmd_play::run(&args![], &missing_play_io);
    assert_error(missing_play, "missing PART", &missing_play_io, 0);

    let nine_win_io = FakeIo::default();
    let nine_win = cmd_win::run(
        &args![
            "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0"
        ],
        &nine_win_io,
    );
    assert_error(nine_win, "too many parts (at most 8)", &nine_win_io, 0);

    let nine_play_io = FakeIo::default();
    let nine_play = cmd_play::run(
        &args![
            "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0"
        ],
        &nine_play_io,
    );
    assert_error(nine_play, "too many parts (at most 8)", &nine_play_io, 0);
}

#[test]
fn multipart_values_are_parsed_left_to_right_before_command_checks() {
    let io = FakeIo::default();
    let result = cmd_sum::run(&args!["oct:70/1", "nim/not-a-size"], &io);
    assert_error(result, "the last digit of \"oct:70\" must not be 0", &io, 0);
}

#[test]
fn period_oct_maximum_and_sum_eight_maximum_parts_are_deterministic() {
    let period_io = FakeIo::default();
    let period = cmd_period::run(&args!["oct:777777777777", "--max", "5000"], &period_io);
    assert_eq!(period, Ok(0));
    assert_eq!(period_io.stdout.borrow().len(), 1);
    assert!(period_io.stderr.borrow().is_empty());
    let period_output = period_io.stdout.borrow();
    let period_lines = period_output[0].lines().collect::<Vec<_>>();
    assert_eq!(period_lines.len(), 5);
    assert_eq!(period_lines[0], "rule        oct:777777777777");
    assert_eq!(period_lines[1], "max         5000");
    assert_eq!(period_lines[2], "preperiod   32");
    assert_eq!(period_lines[3], "period      56");
    assert!(period_lines[4].starts_with("cycle       "));
    let cycle_values = period_lines[4].split_whitespace().collect::<Vec<_>>();
    assert_eq!(cycle_values.len(), 57);
    assert_eq!(cycle_values[0], "cycle");
    assert_eq!(cycle_values[1], "11");
    assert_eq!(cycle_values[2], "10");
    assert_eq!(cycle_values[9], "32");
    assert_eq!(cycle_values[56], "47");
    assert!(period_output[0].ends_with('\n'));
    assert!(!period_output[0].contains('\r'));

    let sum_io = FakeIo::default();
    let sum = cmd_sum::run(
        &args![
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000"
        ],
        &sum_io,
    );
    assert_success(
        sum,
        1,
        &sum_io,
        "parts       8\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\nvalue       0\nwinner      previous\n",
    );
}

#[test]
fn misere_prints_the_supplied_normal_winning_example() {
    let io = FakeIo::default();
    let result = cmd_misere::run(&args!["3", "4", "5"], &io);
    assert_success(
        result,
        0,
        &io,
        "heaps       3 4 5\nbig         3\nnim-sum     2\nwinner      next\nmove        heap 1  take 2  leaves 1\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn misere_prints_the_supplied_mixed_heap_winning_example() {
    let io = FakeIo::default();
    let result = cmd_misere::run(&args!["2", "1"], &io);
    assert_success(
        result,
        0,
        &io,
        "heaps       2 1\nbig         1\nnim-sum     3\nwinner      next\nmove        heap 1  take 2  leaves 0\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn misere_reports_an_all_small_losing_position() {
    let io = FakeIo::default();
    let result = cmd_misere::run(&args!["1", "1", "1"], &io);
    assert_success(
        result,
        1,
        &io,
        "heaps       1 1 1\nbig         0\nnim-sum     1\nwinner      previous\nstatus      no winning move\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn misere_reports_the_winning_move_as_none_for_all_zero_heaps() {
    let io = FakeIo::default();
    let result = cmd_misere::run(&args!["0", "0"], &io);
    assert_success(
        result,
        0,
        &io,
        "heaps       0 0\nbig         0\nnim-sum     0\nwinner      next\nmove        none\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn misere_reads_heap_words_from_stdin_once() {
    let io = FakeIo::with_stdin(b"3 4 5".to_vec());
    let result = cmd_misere::run(&args!["-"], &io);
    assert_success(
        result,
        0,
        &io,
        "heaps       3 4 5\nbig         3\nnim-sum     2\nwinner      next\nmove        heap 1  take 2  leaves 1\n",
    );
    assert_eq!(io.reads.get(), 1);
}

#[test]
fn misere_rejects_more_than_eight_heaps_without_output() {
    let io = FakeIo::default();
    let result = cmd_misere::run(&args!["0", "0", "0", "0", "0", "0", "0", "0", "0"], &io);
    assert_error(result, "too many heaps (at most 8)", &io, 0);
}

#[test]
fn misere_rejects_a_heap_above_its_limit_without_output() {
    let io = FakeIo::default();
    let result = cmd_misere::run(&args!["1001"], &io);
    assert_error(
        result,
        "invalid HEAP \"1001\" (expected an integer from 0 to 1000)",
        &io,
        0,
    );
}

#[test]
fn wythoff_prints_the_supplied_both_piles_winning_example() {
    let io = FakeIo::default();
    let result = cmd_wythoff::run(&args!["4", "6"], &io);
    assert_success(
        result,
        0,
        &io,
        "piles       4 6\nwinner      next\nmove        take 1 from both\nleaves      3 5\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn wythoff_reports_the_supplied_cold_position() {
    let io = FakeIo::default();
    let result = cmd_wythoff::run(&args!["3", "5"], &io);
    assert_success(result, 1, &io, "piles       3 5\nwinner      previous\n");
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn wythoff_preserves_order_for_reversed_winning_piles() {
    let io = FakeIo::default();
    let result = cmd_wythoff::run(&args!["8", "5"], &io);
    assert_success(
        result,
        0,
        &io,
        "piles       8 5\nwinner      next\nmove        take 5 from pile 1\nleaves      3 5\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn wythoff_reports_missing_and_excess_arguments_exactly() {
    let missing_a_io = FakeIo::default();
    let missing_a = cmd_wythoff::run(&args![], &missing_a_io);
    assert_error(missing_a, "missing A", &missing_a_io, 0);

    let missing_b_io = FakeIo::default();
    let missing_b = cmd_wythoff::run(&args!["4"], &missing_b_io);
    assert_error(missing_b, "missing B", &missing_b_io, 0);

    let excess_io = FakeIo::default();
    let excess = cmd_wythoff::run(&args!["4", "6", "8"], &excess_io);
    assert_error(excess, "too many arguments", &excess_io, 0);
}

#[test]
fn wythoff_rejects_out_of_range_and_noncanonical_piles() {
    let high_a_io = FakeIo::default();
    let high_a = cmd_wythoff::run(&args!["20001", "0"], &high_a_io);
    assert_error(
        high_a,
        "invalid A \"20001\" (expected an integer from 0 to 20000)",
        &high_a_io,
        0,
    );

    let high_b_io = FakeIo::default();
    let high_b = cmd_wythoff::run(&args!["0", "20001"], &high_b_io);
    assert_error(
        high_b,
        "invalid B \"20001\" (expected an integer from 0 to 20000)",
        &high_b_io,
        0,
    );

    let noncanonical_io = FakeIo::default();
    let noncanonical = cmd_wythoff::run(&args!["04", "6"], &noncanonical_io);
    assert_error(
        noncanonical,
        "invalid A \"04\" (expected an integer from 0 to 20000)",
        &noncanonical_io,
        0,
    );
}

#[test]
fn pairs_prints_the_default_cold_pairs_exactly() {
    let io = FakeIo::default();
    let result = cmd_pairs::run(&args![], &io);
    assert_success(
        result,
        0,
        &io,
        "max         20\ncount       13\npair        0 0\npair        1 2\npair        3 5\npair        4 7\npair        6 10\npair        8 13\npair        9 15\npair        11 18\npair        12 20\npair        14 23\npair        16 26\npair        17 28\npair        19 31\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn pairs_prints_the_supplied_maximum_ten_example_exactly() {
    let io = FakeIo::default();
    let result = cmd_pairs::run(&args!["--max=10"], &io);
    assert_success(
        result,
        0,
        &io,
        "max         10\ncount       7\npair        0 0\npair        1 2\npair        3 5\npair        4 7\npair        6 10\npair        8 13\npair        9 15\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn pairs_accepts_zero_as_the_inclusive_minimum() {
    let io = FakeIo::default();
    let result = cmd_pairs::run(&args!["--max", "0"], &io);
    assert_success(
        result,
        0,
        &io,
        "max         0\ncount       1\npair        0 0\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn pairs_rejects_unknown_options_invalid_maximum_and_positionals() {
    let unknown_io = FakeIo::default();
    let unknown = cmd_pairs::run(&args!["--other"], &unknown_io);
    assert_error(unknown, "unknown option \"--other\"", &unknown_io, 0);

    let range_io = FakeIo::default();
    let range = cmd_pairs::run(&args!["--max=20001"], &range_io);
    assert_error(
        range,
        "invalid --max \"20001\" (expected an integer from 0 to 20000)",
        &range_io,
        0,
    );

    let positional_io = FakeIo::default();
    let positional = cmd_pairs::run(&args!["10"], &positional_io);
    assert_error(positional, "too many arguments", &positional_io, 0);
}

#[test]
fn nimber_prints_the_supplied_seven_by_nine_example_exactly() {
    let io = FakeIo::default();
    let result = cmd_nimber::run(&args!["7", "9"], &io);
    assert_success(
        result,
        0,
        &io,
        "a           7\nb           9\nsum         14\nproduct     8\ninverse-a   11\ninverse-b   6\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn nimber_prints_zero_operand_and_its_inverse_example_exactly() {
    let io = FakeIo::default();
    let result = cmd_nimber::run(&args!["0", "5"], &io);
    assert_success(
        result,
        0,
        &io,
        "a           0\nb           5\nsum         5\nproduct     0\ninverse-a   none\ninverse-b   12\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn nimber_prints_the_full_width_operand_example_exactly() {
    let io = FakeIo::default();
    let result = cmd_nimber::run(&args!["4294967295", "4294967295"], &io);
    assert_success(
        result,
        0,
        &io,
        "a           4294967295\nb           4294967295\nsum         0\nproduct     2628753316\ninverse-a   12255232\ninverse-b   12255232\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn nimber_rejects_invalid_operands_and_argument_counts() {
    let invalid_a_io = FakeIo::default();
    let invalid_a = cmd_nimber::run(&args!["01", "1"], &invalid_a_io);
    assert_error(
        invalid_a,
        "invalid A \"01\" (expected an integer from 0 to 4294967295)",
        &invalid_a_io,
        0,
    );

    let high_b_io = FakeIo::default();
    let high_b = cmd_nimber::run(&args!["0", "4294967296"], &high_b_io);
    assert_error(
        high_b,
        "invalid B \"4294967296\" (expected an integer from 0 to 4294967295)",
        &high_b_io,
        0,
    );

    let missing_io = FakeIo::default();
    let missing = cmd_nimber::run(&args!["7"], &missing_io);
    assert_error(missing, "missing B", &missing_io, 0);

    let extra_io = FakeIo::default();
    let extra = cmd_nimber::run(&args!["7", "9", "11"], &extra_io);
    assert_error(extra, "too many arguments", &extra_io, 0);
}

#[test]
fn period_cycles_option_changes_how_many_cycles_are_required() {
    let five =
        "rule        sub:2,3\nmax         9\npreperiod   0\nperiod      5\ncycle       0 0 1 1 2\n";

    let default_io = FakeIo::default();
    let default = cmd_period::run(&args!["sub:2,3", "--max", "9"], &default_io);
    assert_success(
        default,
        1,
        &default_io,
        "rule        sub:2,3\nmax         9\nperiod      none\n",
    );

    let two_io = FakeIo::default();
    let two = cmd_period::run(&args!["sub:2,3", "--max", "9", "--cycles", "2"], &two_io);
    assert_success(two, 0, &two_io, five);

    let equals_io = FakeIo::default();
    let equals = cmd_period::run(&args!["--cycles=2", "sub:2,3", "--max=9"], &equals_io);
    assert_success(equals, 0, &equals_io, five);

    let last_io = FakeIo::default();
    let last = cmd_period::run(
        &args!["sub:2,3", "--max", "9", "--cycles", "5", "--cycles", "2"],
        &last_io,
    );
    assert_success(last, 0, &last_io, five);

    let long_pre_io = FakeIo::default();
    let long_pre = cmd_period::run(
        &args!["sub:2,3", "--max", "13", "--cycles", "2"],
        &long_pre_io,
    );
    assert_success(
        long_pre,
        0,
        &long_pre_io,
        "rule        sub:2,3\nmax         13\npreperiod   12\nperiod      1\ncycle       1\n",
    );

    let ten_io = FakeIo::default();
    let ten = cmd_period::run(&args!["oct:77", "--max", "200", "--cycles", "10"], &ten_io);
    assert_success(
        ten,
        0,
        &ten_io,
        "rule        oct:77\nmax         200\npreperiod   71\nperiod      12\ncycle       7 4 1 2 8 1 4 7 2 1 8 2\n",
    );
}

#[test]
fn period_cycles_option_is_range_checked() {
    let low_io = FakeIo::default();
    let low = cmd_period::run(&args!["nim", "--cycles", "1"], &low_io);
    assert_error(
        low,
        "invalid --cycles \"1\" (expected an integer from 2 to 10)",
        &low_io,
        0,
    );

    let high_io = FakeIo::default();
    let high = cmd_period::run(&args!["nim", "--cycles=11"], &high_io);
    assert_error(
        high,
        "invalid --cycles \"11\" (expected an integer from 2 to 10)",
        &high_io,
        0,
    );

    let separate_io = FakeIo::default();
    let separate = cmd_period::run(&args!["sub:2,3", "--cycles", "11"], &separate_io);
    assert_error(
        separate,
        "invalid --cycles \"11\" (expected an integer from 2 to 10)",
        &separate_io,
        0,
    );

    let zero_io = FakeIo::default();
    let zero = cmd_period::run(&args!["nim", "--cycles=03"], &zero_io);
    assert_error(
        zero,
        "invalid --cycles \"03\" (expected an integer from 2 to 10)",
        &zero_io,
        0,
    );

    let missing_io = FakeIo::default();
    let missing = cmd_period::run(&args!["nim", "--cycles"], &missing_io);
    assert_error(missing, "--cycles needs a value", &missing_io, 0);

    let other_io = FakeIo::default();
    let other = cmd_grundy::run(&args!["sub:2,3", "--cycles", "2"], &other_io);
    assert_error(other, "unknown option \"--cycles\"", &other_io, 0);
}
