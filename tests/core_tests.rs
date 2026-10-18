use grundykit::error::GameError;
use grundykit::ints;
use grundykit::io::{self, Io};
use grundykit::options::{self, Spec};
use grundykit::text;
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

    fn assert_silent(&self) {
        assert!(self.stdout.borrow().is_empty());
        assert!(self.stderr.borrow().is_empty());
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

fn max_spec() -> Spec {
    Spec {
        name: "--max",
        lo: 0,
        hi: 20,
        fallback: 7,
    }
}

#[test]
fn game_error_is_a_cloneable_debuggable_equality_value() {
    let error = GameError("bad input".to_owned());
    assert_eq!(error.clone(), GameError("bad input".to_owned()));
    assert_eq!(format!("{error:?}"), "GameError(\"bad input\")");
    assert_ne!(error, GameError("other input".to_owned()));
}

#[test]
fn show_quotes_empty_text() {
    assert_eq!(text::show(""), "\"\"");
}

#[test]
fn show_preserves_printable_punctuation_quotes_and_backslashes() {
    assert_eq!(text::show("a\"b\\c"), "\"a\"b\\c\"");
}

#[test]
fn show_replaces_each_nonprintable_or_non_ascii_scalar_once() {
    assert_eq!(text::show("\n\té😀"), "\"????\"");
}

#[test]
fn line_pads_short_keys_and_adds_no_newline() {
    assert_eq!(text::line("x", "value"), "x           value");
}

#[test]
fn line_does_not_pad_a_ten_character_key() {
    assert_eq!(text::line("1234567890", "value"), "1234567890  value");
}

#[test]
fn line_counts_unicode_key_characters_not_bytes() {
    assert_eq!(text::line("é", "v"), "é           v");
}

#[test]
fn join_formats_an_empty_slice_as_empty_text() {
    assert_eq!(text::join(&[]), "");
}

#[test]
fn join_uses_decimal_values_and_single_spaces() {
    assert_eq!(
        text::join(&[0, 7, 42, u64::MAX]),
        "0 7 42 18446744073709551615"
    );
}

#[test]
fn integer_parser_accepts_zero_and_single_digit_values() {
    assert_eq!(ints::parse("N", "0", 0, 9), Ok(0));
    assert_eq!(ints::parse("N", "3", 0, 9), Ok(3));
}

#[test]
fn integer_parser_accepts_inclusive_bounds_and_ten_digits() {
    assert_eq!(ints::parse("N", "10", 10, 99), Ok(10));
    assert_eq!(ints::parse("N", "99", 10, 99), Ok(99));
    assert_eq!(
        ints::parse("N", "9999999999", 0, 9999999999),
        Ok(9999999999)
    );
}

#[test]
fn integer_parser_rejects_noncanonical_integer_texts() {
    assert_eq!(
        ints::parse("N", "", 0, 99),
        Err(GameError(
            "invalid N \"\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", "-1", 0, 99),
        Err(GameError(
            "invalid N \"-1\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", "+3", 0, 99),
        Err(GameError(
            "invalid N \"+3\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", "007", 0, 99),
        Err(GameError(
            "invalid N \"007\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", "1.5", 0, 99),
        Err(GameError(
            "invalid N \"1.5\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", "12345678901", 0, u64::MAX),
        Err(GameError(
            "invalid N \"12345678901\" (expected an integer from 0 to 18446744073709551615)"
                .to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", " 3", 0, 99),
        Err(GameError(
            "invalid N \"?3\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("N", "١", 0, 99),
        Err(GameError(
            "invalid N \"?\" (expected an integer from 0 to 99)".to_owned()
        ))
    );
}

#[test]
fn integer_parser_rejects_values_outside_inclusive_bounds() {
    assert_eq!(
        ints::parse("size", "9", 10, 20),
        Err(GameError(
            "invalid size \"9\" (expected an integer from 10 to 20)".to_owned()
        ))
    );
    assert_eq!(
        ints::parse("size", "21", 10, 20),
        Err(GameError(
            "invalid size \"21\" (expected an integer from 10 to 20)".to_owned()
        ))
    );
}

#[test]
fn integer_parser_sanitizes_user_text_in_errors() {
    assert_eq!(
        ints::parse("N", "x\né", 0, 9),
        Err(GameError(
            "invalid N \"x??\" (expected an integer from 0 to 9)".to_owned()
        ))
    );
}

#[test]
fn options_accept_inline_and_separate_values_and_preserve_ordinary_arguments() {
    let argv = vec![
        String::from("first"),
        String::from("--max=8"),
        String::from("-"),
        String::from("--max"),
        String::from("9"),
        String::from("two words"),
    ];
    let parsed = options::parse(&argv, &[max_spec()]).expect("valid options");
    assert_eq!(parsed.values.get("--max"), Some(&9));
    assert_eq!(
        parsed.args,
        vec![
            String::from("first"),
            String::from("-"),
            String::from("two words")
        ]
    );
}

#[test]
fn options_populate_fallback_values_for_every_spec() {
    let parsed = options::parse(
        &[],
        &[
            Spec {
                name: "--max",
                lo: 0,
                hi: 20,
                fallback: 7,
            },
            Spec {
                name: "--limit",
                lo: 1,
                hi: 30,
                fallback: 12,
            },
        ],
    )
    .expect("fallbacks");
    assert_eq!(parsed.values.len(), 2);
    assert_eq!(parsed.values.get("--max"), Some(&7));
    assert_eq!(parsed.values.get("--limit"), Some(&12));
    assert!(parsed.args.is_empty());
}

#[test]
fn options_validate_only_the_last_repeated_value() {
    let argv = vec![String::from("--max=bad"), String::from("--max=8")];
    let parsed = options::parse(&argv, &[max_spec()]).expect("last value is valid");
    assert_eq!(parsed.values.get("--max"), Some(&8));
}

#[test]
fn options_reject_the_last_repeated_value_even_after_a_valid_one() {
    let argv = vec![String::from("--max=8"), String::from("--max=21")];
    assert_eq!(
        options::parse(&argv, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("invalid --max \"21\" (expected an integer from 0 to 20)".to_owned())
    );
}

#[test]
fn options_consume_a_dash_prefixed_next_argument_as_the_value() {
    let argv = vec![String::from("--max"), String::from("--help")];
    assert_eq!(
        options::parse(&argv, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("invalid --max \"--help\" (expected an integer from 0 to 20)".to_owned())
    );
}

#[test]
fn options_split_inline_values_at_only_the_first_equals_sign() {
    let argv = vec![String::from("--max=8=9")];
    assert_eq!(
        options::parse(&argv, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("invalid --max \"8=9\" (expected an integer from 0 to 20)".to_owned())
    );
}

#[test]
fn options_report_an_empty_inline_value_through_integer_validation() {
    let argv = vec![String::from("--max=")];
    assert_eq!(
        options::parse(&argv, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("invalid --max \"\" (expected an integer from 0 to 20)".to_owned())
    );
}

#[test]
fn options_report_unknown_double_dash_arguments_as_whole_arguments() {
    let dash = vec![String::from("--")];
    assert_eq!(
        options::parse(&dash, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("unknown option \"--\"".to_owned())
    );
    let help = vec![String::from("--help")];
    assert_eq!(
        options::parse(&help, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("unknown option \"--help\"".to_owned())
    );
    let inline = vec![String::from("--unknown=value")];
    assert_eq!(
        options::parse(&inline, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("unknown option \"--unknown=value\"".to_owned())
    );
}

#[test]
fn options_report_a_final_recognized_option_without_a_value() {
    let argv = vec![String::from("--max")];
    assert_eq!(
        options::parse(&argv, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("--max needs a value".to_owned())
    );
}

#[test]
fn options_report_structural_errors_before_deferred_numeric_errors() {
    let argv = vec![String::from("--max=bad"), String::from("--unknown")];
    assert_eq!(
        options::parse(&argv, &[max_spec()])
            .err()
            .expect("error expected"),
        GameError("unknown option \"--unknown\"".to_owned())
    );
}

#[test]
fn options_validate_invalid_values_in_specification_order() {
    let argv = vec![
        String::from("--second=bad"),
        String::from("--first=also-bad"),
    ];
    let specs = [
        Spec {
            name: "--first",
            lo: 0,
            hi: 5,
            fallback: 0,
        },
        Spec {
            name: "--second",
            lo: 0,
            hi: 5,
            fallback: 0,
        },
    ];
    assert_eq!(
        options::parse(&argv, &specs).err().expect("error expected"),
        GameError("invalid --first \"also-bad\" (expected an integer from 0 to 5)".to_owned())
    );
}

#[test]
fn io_words_reads_and_rejects_exact_malformed_utf8_bytes() {
    let io = FakeIo::with_stdin(vec![0x6e, 0x69, 0x6d, 0x2f, 0x33, 0xff, 0x0a]);
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Err(GameError("<stdin>: input is not valid UTF-8".to_owned()))
    );
    assert_eq!(io.reads.get(), 1);
    io.assert_silent();
}

#[test]
fn io_words_removes_one_leading_bom_and_splits_the_four_ascii_separators() {
    let input = "\u{feff}nim/3  sub:1,3\toct:77\r\n";
    let io = FakeIo::with_stdin(input.as_bytes().to_vec());
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Ok(vec![
            String::from("nim/3"),
            String::from("sub:1,3"),
            String::from("oct:77")
        ])
    );
    assert_eq!(io.reads.get(), 1);
    io.assert_silent();
}

#[test]
fn io_words_preserves_second_and_interior_boms() {
    let io = FakeIo::with_stdin("\u{feff}\u{feff}x a\u{feff}b".as_bytes().to_vec());
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Ok(vec![String::from("\u{feff}x"), String::from("a\u{feff}b")])
    );
    io.assert_silent();
}

#[test]
fn io_words_rejects_empty_stdin() {
    let io = FakeIo::with_stdin(Vec::new());
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Err(GameError("<stdin>: no input".to_owned()))
    );
    assert_eq!(io.reads.get(), 1);
    io.assert_silent();
}

#[test]
fn io_words_rejects_stdin_containing_only_allowed_separators() {
    let io = FakeIo::with_stdin(b" \t\r\n \t".to_vec());
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Err(GameError("<stdin>: no input".to_owned()))
    );
    io.assert_silent();
}

#[test]
fn io_words_rejects_bom_only_stdin_after_removing_the_bom() {
    let io = FakeIo::with_stdin("\u{feff}".as_bytes().to_vec());
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Err(GameError("<stdin>: no input".to_owned()))
    );
    io.assert_silent();
}

#[test]
fn io_words_keeps_nul_vertical_tab_form_feed_and_non_ascii_whitespace() {
    let io = FakeIo::with_stdin("a\0b\u{000b}c\u{000c}d\u{00a0}e".as_bytes().to_vec());
    let args = vec![String::from("-")];
    assert_eq!(
        io::words(&io, &args),
        Ok(vec![String::from("a\0b\u{000b}c\u{000c}d\u{00a0}e")])
    );
    io.assert_silent();
}

#[test]
fn io_words_returns_empty_arguments_without_reading_stdin() {
    let io = FakeIo::with_stdin(b"unused".to_vec());
    assert_eq!(io::words(&io, &[]), Ok(Vec::new()));
    assert_eq!(io.reads.get(), 0);
    io.assert_silent();
}

#[test]
fn io_words_preserves_non_stdin_arguments_without_reading_or_writing() {
    let io = FakeIo::with_stdin(b"unused".to_vec());
    let args = vec![
        String::from("-"),
        String::from("embedded whitespace"),
        String::from("second"),
    ];
    assert_eq!(io::words(&io, &args), Ok(args));
    assert_eq!(io.reads.get(), 0);
    io.assert_silent();
}

#[test]
fn show_sanitizes_the_future_module_placeholder_line_ending() {
    assert_eq!(
        text::show("// Implemented by its owning feature.\n"),
        "\"//?Implemented?by?its?owning?feature.?\""
    );
}
