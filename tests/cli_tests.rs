use grundykit::cli::{self, USAGE};
use grundykit::io::Io;
use std::cell::{Cell, RefCell};
use std::io::Write;
use std::process::{Command, Stdio};

#[derive(Default)]
struct FakeIo {
    stdin: Vec<u8>,
    reads: Cell<usize>,
    stdout: RefCell<Vec<String>>,
    stderr: RefCell<Vec<String>>,
}

impl FakeIo {
    fn with_stdin(stdin: &[u8]) -> Self {
        Self {
            stdin: stdin.to_vec(),
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

fn args(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_owned()).collect()
}

fn assert_success(code: i32, expected_code: i32, io: &FakeIo, expected_stdout: &str) {
    assert_eq!(code, expected_code);
    assert_eq!(io.stdout.borrow().as_slice(), &[expected_stdout.to_owned()]);
    assert!(io.stderr.borrow().is_empty());
}

fn assert_error(code: i32, io: &FakeIo, message: &str) {
    assert_eq!(code, 2);
    assert!(io.stdout.borrow().is_empty());
    assert_eq!(
        io.stderr.borrow().as_slice(),
        &[format!("grundykit: {message}\n")]
    );
}

#[test]
fn usage_is_the_exact_ten_line_contract_without_a_final_newline() {
    assert_eq!(
        USAGE,
        "usage: grundykit grundy [--max N] RULE\n       grundykit moves PART\n       grundykit sum PART...\n       grundykit win PART...\n       grundykit play PART...\n       grundykit period [--max N] [--cycles N] RULE\n       grundykit misere HEAP...\n       grundykit wythoff A B\n       grundykit pairs [--max N]\n       grundykit nimber A B"
    );
    assert!(!USAGE.ends_with('\n'));
}

#[test]
fn sole_help_prints_usage_once_without_reading_stdin() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["--help"]), &io);

    assert_success(code, 0, &io, &format!("{USAGE}\n"));
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn missing_command_has_the_exact_diagnostic_and_exit_code() {
    let io = FakeIo::default();

    let code = cli::run(&args(&[]), &io);

    assert_error(
        code,
        &io,
        "missing command (grundy, moves, sum, win, play, period, misere, wythoff, pairs or nimber)",
    );
}

#[test]
fn unknown_command_text_sanitizes_spaces_controls_and_non_ascii() {
    let space_io = FakeIo::default();
    let space_code = cli::run(&args(&["bad name"]), &space_io);
    assert_error(space_code, &space_io, "unknown command \"bad?name\"");

    let control_io = FakeIo::default();
    let control_code = cli::run(&args(&["bad\nname"]), &control_io);
    assert_error(control_code, &control_io, "unknown command \"bad?name\"");

    let non_ascii_io = FakeIo::default();
    let non_ascii_code = cli::run(&args(&["é"]), &non_ascii_io);
    assert_error(non_ascii_code, &non_ascii_io, "unknown command \"?\"");
}

#[test]
fn help_is_not_special_when_it_is_not_the_only_argument() {
    let trailing_io = FakeIo::default();
    let trailing_code = cli::run(&args(&["--help", "nim"]), &trailing_io);
    assert_error(trailing_code, &trailing_io, "unknown command \"--help\"");

    let option_io = FakeIo::default();
    let option_code = cli::run(&args(&["moves", "--help"]), &option_io);
    assert_error(option_code, &option_io, "unknown option \"--help\"");
}

#[test]
fn grundy_dispatch_prints_the_default_nim_table_exactly() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["grundy", "nim"]), &io);

    assert_eq!(code, 0);
    assert_eq!(io.stdout.borrow().len(), 1);
    assert!(io.stderr.borrow().is_empty());
    let output = io.stdout.borrow();
    let lines = output[0].lines().collect::<Vec<_>>();
    assert_eq!(lines.len(), 5);
    assert_eq!(lines[0], "rule        nim");
    assert_eq!(lines[1], "max         20");
    let values = lines[2].split_whitespace().collect::<Vec<_>>();
    assert_eq!(values.len(), 22);
    assert_eq!(values[0], "values");
    assert_eq!(values[1], "0");
    assert_eq!(values[2], "1");
    assert_eq!(values[3], "2");
    assert_eq!(values[11], "10");
    assert_eq!(values[21], "20");
    assert_eq!(lines[3], "zeros       0");
    assert_eq!(lines[4], "largest     20");
    assert!(output[0].ends_with('\n'));
    assert!(!output[0].contains('\r'));
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn moves_dispatch_prints_the_ordered_kayles_moves_exactly() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["moves", "oct:77/5"]), &io);

    assert_success(
        code,
        0,
        &io,
        "part        oct:77/5\nvalue       4\nmoves       5\nmove        take 1  leaves 4  value 1\nmove        take 1  leaves 1+3  value 2\nmove        take 1  leaves 2+2  value 0\nmove        take 2  leaves 3  value 3\nmove        take 2  leaves 1+2  value 3\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn sum_dispatch_prints_a_winning_position_exactly() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["sum", "nim/3", "nim/4", "nim/5"]), &io);

    assert_success(
        code,
        0,
        &io,
        "parts       3\npart        nim/3  value 3\npart        nim/4  value 4\npart        nim/5  value 5\nvalue       2\nwinner      next\n",
    );
    assert_eq!(io.reads.get(), 0);
}

#[test]
fn sum_dispatch_preserves_the_negative_answer_exit_code() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["sum", "nim/2", "nim/2"]), &io);

    assert_success(
        code,
        1,
        &io,
        "parts       2\npart        nim/2  value 2\npart        nim/2  value 2\nvalue       0\nwinner      previous\n",
    );
}

#[test]
fn win_dispatch_prints_the_first_winning_move_exactly() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["win", "nim/3", "nim/4", "nim/5"]), &io);

    assert_success(
        code,
        0,
        &io,
        "value       2\npart        1\ntake        2\nleaves      1\nafter       nim/1 nim/4 nim/5\n",
    );
}

#[test]
fn win_dispatch_reports_no_winning_move_with_exit_one() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["win", "nim/2", "nim/2"]), &io);

    assert_success(code, 1, &io, "value       0\nstatus      no winning move\n");
}

#[test]
fn play_dispatch_emits_the_literal_complete_transcript() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["play", "nim/1", "nim/2", "nim/3"]), &io);

    assert_success(
        code,
        0,
        &io,
        "start       nim/1 nim/2 nim/3\nmove        1  A  part 1  take 1  ->  nim/2 nim/3\nmove        2  B  part 2  take 1  ->  nim/2 nim/2\nmove        3  A  part 1  take 1  ->  nim/1 nim/2\nmove        4  B  part 2  take 1  ->  nim/1 nim/1\nmove        5  A  part 1  take 1  ->  nim/1\nmove        6  B  part 1  take 1  ->  (empty)\nmoves       6\nwinner      B\n",
    );
}

#[test]
fn period_dispatch_prints_the_supplied_kayles_period() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["period", "oct:77", "--max", "200"]), &io);

    assert_success(
        code,
        0,
        &io,
        "rule        oct:77\nmax         200\npreperiod   71\nperiod      12\ncycle       7 4 1 2 8 1 4 7 2 1 8 2\n",
    );
}

#[test]
fn period_dispatch_reports_no_period_with_exit_one() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["period", "sub:2,3", "--max=13"]), &io);

    assert_success(
        code,
        1,
        &io,
        "rule        sub:2,3\nmax         13\nperiod      none\n",
    );
}

#[test]
fn misere_dispatch_prints_a_winning_move_and_a_losing_answer() {
    let winning_io = FakeIo::default();
    let winning_code = cli::run(&args(&["misere", "3", "4", "5"]), &winning_io);
    assert_success(
        winning_code,
        0,
        &winning_io,
        "heaps       3 4 5\nbig         3\nnim-sum     2\nwinner      next\nmove        heap 1  take 2  leaves 1\n",
    );

    let losing_io = FakeIo::default();
    let losing_code = cli::run(&args(&["misere", "1", "1", "1"]), &losing_io);
    assert_success(
        losing_code,
        1,
        &losing_io,
        "heaps       1 1 1\nbig         0\nnim-sum     1\nwinner      previous\nstatus      no winning move\n",
    );
}

#[test]
fn wythoff_dispatch_prints_a_winning_move_and_cold_position() {
    let winning_io = FakeIo::default();
    let winning_code = cli::run(&args(&["wythoff", "4", "6"]), &winning_io);
    assert_success(
        winning_code,
        0,
        &winning_io,
        "piles       4 6\nwinner      next\nmove        take 1 from both\nleaves      3 5\n",
    );

    let cold_io = FakeIo::default();
    let cold_code = cli::run(&args(&["wythoff", "3", "5"]), &cold_io);
    assert_success(
        cold_code,
        1,
        &cold_io,
        "piles       3 5\nwinner      previous\n",
    );
}

#[test]
fn pairs_dispatch_accepts_an_inline_maximum_and_includes_zero() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["pairs", "--max=0"]), &io);

    assert_success(
        code,
        0,
        &io,
        "max         0\ncount       1\npair        0 0\n",
    );
}

#[test]
fn nimber_dispatch_handles_the_maximum_operands_exactly() {
    let io = FakeIo::default();

    let code = cli::run(&args(&["nimber", "4294967295", "4294967295"]), &io);

    assert_success(
        code,
        0,
        &io,
        "a           4294967295\nb           4294967295\nsum         0\nproduct     2628753316\ninverse-a   12255232\ninverse-b   12255232\n",
    );
}

#[test]
fn adapter_errors_are_prefixed_once_without_partial_output() {
    let part_io = FakeIo::default();
    let part_code = cli::run(&args(&["moves", "nim"]), &part_io);
    assert_error(
        part_code,
        &part_io,
        "invalid part \"nim\" (expected RULE/SIZE such as nim/5 or oct:77/9)",
    );

    let size_io = FakeIo::default();
    let size_code = cli::run(&args(&["moves", "nim/3/4"]), &size_io);
    assert_error(
        size_code,
        &size_io,
        "invalid size \"3/4\" (expected an integer from 0 to 2000)",
    );

    let rule_io = FakeIo::default();
    let rule_code = cli::run(&args(&["grundy", "oct:70"]), &rule_io);
    assert_error(
        rule_code,
        &rule_io,
        "the last digit of \"oct:70\" must not be 0",
    );
}

#[test]
fn validation_precedence_is_options_then_arity_then_values_then_command_checks() {
    let option_io = FakeIo::default();
    let option_code = cli::run(&args(&["grundy", "--bad"]), &option_io);
    assert_error(option_code, &option_io, "unknown option \"--bad\"");

    let arity_io = FakeIo::default();
    let arity_code = cli::run(&args(&["nimber", "not-a-number"]), &arity_io);
    assert_error(arity_code, &arity_io, "missing B");

    let values_io = FakeIo::default();
    let values_code = cli::run(&args(&["play", "nim/41", "not-a-part"]), &values_io);
    assert_error(
        values_code,
        &values_io,
        "invalid part \"not-a-part\" (expected RULE/SIZE such as nim/5 or oct:77/9)",
    );

    let command_check_io = FakeIo::default();
    let command_check_code = cli::run(&args(&["play", "nim/41"]), &command_check_io);
    assert_error(
        command_check_code,
        &command_check_io,
        "total size 41 is above 40 (too large for play)",
    );
}

#[test]
fn arity_limits_precede_value_parsing_and_empty_stdin_is_reported() {
    let too_many_io = FakeIo::default();
    let too_many_code = cli::run(
        &args(&[
            "sum", "bad", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0", "nim/0",
            "nim/0",
        ]),
        &too_many_io,
    );
    assert_error(too_many_code, &too_many_io, "too many parts (at most 8)");

    let empty_io = FakeIo::with_stdin(b" \t\r\n");
    let empty_code = cli::run(&args(&["sum", "-"]), &empty_io);
    assert_error(empty_code, &empty_io, "<stdin>: no input");
    assert_eq!(empty_io.reads.get(), 1);
}

#[test]
fn multipart_stdin_keeps_bom_and_crlf_tab_parsing_inside_the_cli() {
    let io = FakeIo::with_stdin(b"\xef\xbb\xbf nim/3\t nim/4\r\nnim/5 ");

    let code = cli::run(&args(&["sum", "-"]), &io);

    assert_success(
        code,
        0,
        &io,
        "parts       3\npart        nim/3  value 3\npart        nim/4  value 4\npart        nim/5  value 5\nvalue       2\nwinner      next\n",
    );
    assert_eq!(io.reads.get(), 1);
}

#[test]
fn maximum_range_period_sum_and_nimber_commands_keep_literal_results() {
    let period_io = FakeIo::default();
    let period_code = cli::run(
        &args(&["period", "oct:777777777777", "--max", "5000"]),
        &period_io,
    );
    assert_eq!(period_code, 0);
    assert_eq!(period_io.stdout.borrow().len(), 1);
    assert!(period_io.stderr.borrow().is_empty());
    let period_output = period_io.stdout.borrow();
    let period_lines = period_output[0].lines().collect::<Vec<_>>();
    assert_eq!(period_lines.len(), 5);
    assert_eq!(period_lines[0], "rule        oct:777777777777");
    assert_eq!(period_lines[1], "max         5000");
    assert_eq!(period_lines[2], "preperiod   32");
    assert_eq!(period_lines[3], "period      56");
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
    let sum_code = cli::run(
        &args(&[
            "sum",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
            "oct:777777777777/2000",
        ]),
        &sum_io,
    );
    assert_success(
        sum_code,
        1,
        &sum_io,
        "parts       8\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\npart        oct:777777777777/2000  value 32\nvalue       0\nwinner      previous\n",
    );

    let nimber_io = FakeIo::default();
    let nimber_code = cli::run(&args(&["nimber", "4294967295", "4294967295"]), &nimber_io);
    assert_success(
        nimber_code,
        0,
        &nimber_io,
        "a           4294967295\nb           4294967295\nsum         0\nproduct     2628753316\ninverse-a   12255232\ninverse-b   12255232\n",
    );
}

#[test]
fn malformed_utf8_on_real_stdin_is_the_only_process_test() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_grundykit"))
        .args(["sum", "-"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("launch grundykit");

    child
        .stdin
        .as_mut()
        .expect("stdin pipe")
        .write_all(&[0x6e, 0x69, 0x6d, 0x2f, 0x33, 0xff, 0x0a])
        .expect("write malformed UTF-8 input");
    drop(child.stdin.take());

    let output = child.wait_with_output().expect("wait for grundykit");
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(output.stdout, b"");
    assert_eq!(
        output.stderr,
        b"grundykit: <stdin>: input is not valid UTF-8\n"
    );
}
