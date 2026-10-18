use grundykit::cli;
use grundykit::io::Io;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

const README: &str = include_str!("../README.md");

#[derive(Default)]
struct FakeIo {
    stdin: Vec<u8>,
    stdout: RefCell<Vec<String>>,
    stderr: RefCell<Vec<String>>,
}

impl Io for FakeIo {
    fn read_stdin(&self) -> Vec<u8> {
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

fn fenced_text_block_after(markdown: &str, offset: usize) -> (String, usize) {
    let opening = "```text\n";
    let opening_at = markdown[offset..]
        .find(&opening)
        .map(|index| offset + index)
        .expect("a fenced text block should follow");
    let content_start = opening_at + opening.len();
    let closing_at = markdown[content_start..]
        .find("\n```")
        .map(|index| content_start + index)
        .expect("the text block should close");
    (
        markdown[content_start..closing_at].to_owned(),
        closing_at + 4,
    )
}

fn command_output(markdown: &str, command: &str) -> String {
    let marker = format!("Command: `{command}`\n");
    assert_eq!(markdown.matches(&marker).count(), 1);
    let command_at = markdown.find(&marker).expect("example command");
    let (output, _) = fenced_text_block_after(markdown, command_at + marker.len());
    format!("{output}\n")
}

fn section_between<'a>(markdown: &'a str, start: &str, end: &str) -> &'a str {
    let start_at = markdown.find(start).expect("required README heading");
    let content_at = start_at + start.len();
    let end_at = markdown[content_at..]
        .find(end)
        .map(|index| content_at + index)
        .expect("following README heading");
    &markdown[content_at..end_at]
}

fn bullet_paths(section: &str) -> Vec<String> {
    section
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            assert!(
                line.starts_with("- `"),
                "layout line must be a bullet: {line}"
            );
            let rest = &line[3..];
            let path_end = rest
                .find('`')
                .expect("bullet path must be enclosed in backticks");
            let path = &rest[..path_end];
            let description = rest[path_end + 1..]
                .strip_prefix(" - ")
                .expect("path must be followed by a description");
            assert!(description.ends_with('.'));
            assert_eq!(description.matches('.').count(), 1);
            path.to_owned()
        })
        .collect()
}

fn direct_rust_files(root: &Path, directory: &str) -> Vec<String> {
    let directory_path = root.join(directory);
    let mut paths = std::fs::read_dir(&directory_path)
        .expect("source directory should be readable")
        .map(|entry| entry.expect("directory entry should be readable").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "rs"))
        .map(|path| {
            path.strip_prefix(root)
                .expect("file should be under the project root")
                .to_string_lossy()
                .replace('\\', "/")
        })
        .collect::<Vec<_>>();
    paths.sort();
    paths
}

const USAGE: &str = "usage: grundykit grundy [--max N] RULE\n       grundykit moves PART\n       grundykit sum PART...\n       grundykit win PART...\n       grundykit play PART...\n       grundykit period [--max N] RULE\n       grundykit misere HEAP...\n       grundykit wythoff A B\n       grundykit pairs [--max N]\n       grundykit nimber A B";

const MOVES_OUTPUT: &str = "part        oct:77/5\nvalue       4\nmoves       5\nmove        take 1  leaves 4  value 1\nmove        take 1  leaves 1+3  value 2\nmove        take 1  leaves 2+2  value 0\nmove        take 2  leaves 3  value 3\nmove        take 2  leaves 1+2  value 3\n";

const WIN_OUTPUT: &str = "value       2\npart        1\ntake        2\nleaves      2+5\nafter       oct:77/2 oct:77/5 nim/6 sub:1,3,4/7\n";

const PERIOD_OUTPUT: &str = "rule        oct:77\nmax         200\npreperiod   71\nperiod      12\ncycle       7 4 1 2 8 1 4 7 2 1 8 2\n";

#[test]
fn readme_usage_is_the_exact_contract_and_help_output() {
    let usage_section = section_between(README, "## Usage\n", "\n## Rules");
    let (readme_usage, _) = fenced_text_block_after(usage_section, 0);
    assert_eq!(readme_usage, USAGE);
    assert_eq!(USAGE.lines().count(), 10);

    let io = FakeIo::default();
    let code = cli::run(&args(&["--help"]), &io);

    assert_eq!(code, 0);
    assert_eq!(io.stdout.borrow().as_slice(), &[format!("{USAGE}\n")]);
    assert!(io.stderr.borrow().is_empty());
}

#[test]
fn readme_moves_example_matches_its_literal_contract_and_cli() {
    let examples = section_between(README, "### Examples\n", "\nThe optional `--max`");
    assert_eq!(examples.matches("Command: `").count(), 3);
    assert_eq!(examples.matches("```text\n").count(), 3);
    let output = command_output(examples, "grundykit moves oct:77/5");
    assert_eq!(output, MOVES_OUTPUT);

    let io = FakeIo::default();
    let code = cli::run(&args(&["moves", "oct:77/5"]), &io);

    assert_eq!(code, 0);
    assert_eq!(io.stdout.borrow().as_slice(), &[output]);
    assert!(io.stderr.borrow().is_empty());
}

#[test]
fn readme_win_example_matches_its_literal_contract_and_cli() {
    let examples = section_between(README, "### Examples\n", "\nThe optional `--max`");
    let output = command_output(examples, "grundykit win oct:77/9 nim/6 sub:1,3,4/7");
    assert_eq!(output, WIN_OUTPUT);

    let io = FakeIo::default();
    let code = cli::run(&args(&["win", "oct:77/9", "nim/6", "sub:1,3,4/7"]), &io);

    assert_eq!(code, 0);
    assert_eq!(io.stdout.borrow().as_slice(), &[output]);
    assert!(io.stderr.borrow().is_empty());
}

#[test]
fn readme_period_example_matches_its_literal_contract_and_cli() {
    let examples = section_between(README, "### Examples\n", "\nThe optional `--max`");
    let output = command_output(examples, "grundykit period oct:77 --max 200");
    assert_eq!(output, PERIOD_OUTPUT);

    let io = FakeIo::default();
    let code = cli::run(&args(&["period", "oct:77", "--max", "200"]), &io);

    assert_eq!(code, 0);
    assert_eq!(io.stdout.borrow().as_slice(), &[output]);
    assert!(io.stderr.borrow().is_empty());
}

#[test]
fn readme_has_the_required_sections_commands_limits_and_behavior() {
    for heading in [
        "## Install",
        "## Usage",
        "## Rules",
        "## Commands",
        "## Exit codes",
        "## Layout",
        "## Development",
    ] {
        assert!(README.contains(heading), "missing {heading}");
    }
    for command in [
        "`grundy [--max N] RULE`",
        "`moves PART`",
        "`sum PART...`",
        "`win PART...`",
        "`play PART...`",
        "`period [--max N] RULE`",
        "`misere HEAP...`",
        "`wythoff A B`",
        "`pairs [--max N]`",
        "`nimber A B`",
    ] {
        assert!(README.contains(command), "missing command {command}");
    }

    for detail in [
        "Install a Rust toolchain",
        "cargo build --release",
        "target/release/grundykit.exe",
        "student",
        "puzzle makers",
        "`nim`",
        "`sub:T1,T2,...`",
        "one to eight comma-separated takes, each from 1 to 1000",
        "canonical form sorts them in ascending order and removes duplicates",
        "`sub:01` is invalid",
        "`oct:D1D2...`",
        "one to twelve digits from 0 through 7, with a nonzero final digit",
        "`oct:70` is invalid",
        "its size is from 0 through 2000",
        "one to eight arguments",
        "one `-` argument to read words from standard input",
        "spaces, tabs and line endings separate words, one leading UTF-8 byte-order mark is ignored",
        "defaults to 20 for `grundy` (range 0–2000)",
        "300 for `period` (range 1–5000)",
        "20 for `pairs` (range 0–20000)",
        "total size of all parts must not exceed 40",
        "heaps of size 0 through 1000",
        "each from 0 through 20000",
        "operands from 0 through 4294967295",
        "first winning move",
        "in the finite sequence through the chosen maximum",
        "All program output lines end with LF",
        "`0`: the command completed successfully",
        "`1`: a negative answer for `sum`, `win`, `period`, `misere`, or `wythoff`",
        "`2`: an input or command error",
        "cargo fmt --check",
        "cargo clippy -- -D warnings",
        "cargo test",
    ] {
        assert!(
            README.contains(detail),
            "missing documentation detail: {detail}"
        );
    }
}

#[test]
fn readme_layout_is_the_ordered_complete_file_inventory() {
    let layout = section_between(README, "## Layout\n", "\n## Development");
    let source = section_between(layout, "### Source\n", "\n### Tests\n");
    let tests = layout.split_once("### Tests\n").expect("tests heading").1;

    let expected_source = [
        "src/error.rs",
        "src/text.rs",
        "src/ints.rs",
        "src/options.rs",
        "src/io.rs",
        "src/rule.rs",
        "src/moves.rs",
        "src/position.rs",
        "src/nimber.rs",
        "src/table.rs",
        "src/solve.rs",
        "src/period.rs",
        "src/play.rs",
        "src/misere.rs",
        "src/wythoff.rs",
        "src/cmd_grundy.rs",
        "src/cmd_moves.rs",
        "src/cmd_sum.rs",
        "src/cmd_win.rs",
        "src/cmd_play.rs",
        "src/cmd_period.rs",
        "src/cmd_misere.rs",
        "src/cmd_wythoff.rs",
        "src/cmd_pairs.rs",
        "src/cmd_nimber.rs",
        "src/cli.rs",
        "src/main.rs",
        "src/lib.rs",
    ];
    let expected_tests = [
        "tests/core_tests.rs",
        "tests/rule_tests.rs",
        "tests/nimber_tests.rs",
        "tests/table_tests.rs",
        "tests/period_tests.rs",
        "tests/play_tests.rs",
        "tests/misere_tests.rs",
        "tests/wythoff_tests.rs",
        "tests/command_tests.rs",
        "tests/cli_tests.rs",
        "tests/docs_tests.rs",
    ];

    let source_paths = bullet_paths(source);
    let test_paths = bullet_paths(tests);
    assert_eq!(source_paths, expected_source.map(str::to_owned).to_vec());
    assert_eq!(test_paths, expected_tests.map(str::to_owned).to_vec());

    let all_paths = source_paths
        .iter()
        .chain(test_paths.iter())
        .cloned()
        .collect::<Vec<_>>();
    assert_eq!(all_paths.len(), 39);
    assert_eq!(all_paths.iter().collect::<BTreeSet<_>>().len(), 39);

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for path in &all_paths {
        assert!(
            root.join(path).is_file(),
            "listed file should exist: {path}"
        );
    }

    let listed_sources = source_paths.iter().cloned().collect::<BTreeSet<_>>();
    let listed_tests = test_paths.iter().cloned().collect::<BTreeSet<_>>();
    assert_eq!(
        listed_sources,
        direct_rust_files(&root, "src").into_iter().collect()
    );
    assert_eq!(
        listed_tests,
        direct_rust_files(&root, "tests").into_iter().collect()
    );
}
