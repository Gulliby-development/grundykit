<p align="center"><img src="docs/logo.svg" width="96" alt="" /></p>

# grundykit

grundykit is an offline command-line calculator for impartial heap games, including Nim, subtraction games, Kayles and other octal games. It computes exact Grundy values, winners and winning moves, and can play a position out. It is for students of combinatorial game theory, puzzle makers, and anyone who wants exact answers without writing a search.

[![CI](https://github.com/Gulliby-development/grundykit/actions/workflows/ci.yml/badge.svg)](https://github.com/Gulliby-development/grundykit/actions/workflows/ci.yml) [![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Install

Install a Rust toolchain, then build the release executable:

```text
cargo build --release
```

The executable is `target/release/grundykit` on platforms without an executable suffix and `target/release/grundykit.exe` on Windows. grundykit is self-contained and works offline.

## Usage

```text
usage: grundykit grundy [--max N] RULE
       grundykit moves PART
       grundykit sum PART...
       grundykit win PART...
       grundykit play PART...
       grundykit period [--max N] [--cycles N] RULE
       grundykit misere HEAP...
       grundykit wythoff A B
       grundykit pairs [--max N]
       grundykit nimber A B
```

A rule is a rule text; a part is a rule and heap size written as `RULE/SIZE`; heap and pile arguments are decimal integers. Commands accepting `PART...` or `HEAP...` take one to eight arguments, or one `-` argument to read words from standard input. In the latter form, spaces, tabs and line endings separate words, one leading UTF-8 byte-order mark is ignored, and empty input is an error. A `-` alongside other arguments is an ordinary word.

### Examples

Command: `grundykit moves oct:77/5`

```text
part        oct:77/5
value       4
moves       5
move        take 1  leaves 4  value 1
move        take 1  leaves 1+3  value 2
move        take 1  leaves 2+2  value 0
move        take 2  leaves 3  value 3
move        take 2  leaves 1+2  value 3
```

Command: `grundykit win oct:77/9 nim/6 sub:1,3,4/7`

```text
value       2
part        1
take        2
leaves      2+5
after       oct:77/2 oct:77/5 nim/6 sub:1,3,4/7
```

Command: `grundykit period oct:77 --max 200`

```text
rule        oct:77
max         200
preperiod   71
period      12
cycle       7 4 1 2 8 1 4 7 2 1 8 2
```

The optional `--max` defaults to 20 for `grundy` (range 0–2000), 300 for `period` (range 1–5000), and 20 for `pairs` (range 0–20000). The optional `--cycles` of `period` defaults to 3 (range 2–10).

## Rules

A rule is `nim`, `sub:T1,T2,...`, or `oct:D1D2...`. Nim allows taking any positive number of tokens up to the heap size. A subtraction rule has one to eight comma-separated takes, each from 1 to 1000. Takes are decimal integers without leading zeroes; the canonical form sorts them in ascending order and removes duplicates, so `sub:4,1,3,3` canonicalizes to `sub:1,3,4`, while `sub:01` is invalid. An octal rule has one to twelve digits from 0 through 7, with a nonzero final digit, so `oct:70` is invalid. For each number of tokens removed, the digit's 1 bit allows leaving no heap, its 2 bit allows leaving one heap, and its 4 bit allows splitting the remainder into two non-empty heaps; for example, `oct:77` is Kayles. A part uses `RULE/SIZE`, such as `oct:77/9`, and its size is from 0 through 2000.

## Commands

`grundy [--max N] RULE` lists the Grundy values for heap sizes from zero through the selected maximum, along with the zero-valued sizes and largest value.

`moves PART` shows the part's Grundy value and every legal move, including the heaps left behind and the move's value.

`sum PART...` reports each part's value, their nim-sum, and whether the next or previous player wins.

`win PART...` finds the first winning move in deterministic order, or reports that there is no winning move. It tries parts in input order and moves in their defined order: increasing take, then no heap, one heap, and splits by increasing smaller part.

`play PART...` plays a normal-play position to completion: the next player takes a winning move when one exists, otherwise the first legal move. The total size of all parts must not exceed 40.

`period [--max N] [--cycles N] RULE` searches the computed Grundy sequence for a repeating suffix with at least N full cycles (`--cycles`, default 3). For example `period sub:2,3 --max 9` finds no period, while adding `--cycles 2` reports the period 5. Any reported period is a pattern detected in the finite sequence through the chosen maximum; it is not a proof about an uncomputed infinite sequence.

`misere HEAP...` solves misere Nim, where taking the last token loses, for one to eight heaps of size 0 through 1000. It reports the winner and, when one exists, the first winning move in heap order.

`wythoff A B` solves Wythoff's game for two piles, each from 0 through 20000, and reports a winning move when the position is not cold.

`pairs [--max N]` lists the cold pairs of Wythoff's game whose smaller pile is at most the selected maximum.

`nimber A B` reports the nim-sum, nim product and multiplicative inverse (when nonzero) for operands from 0 through 4294967295.

The moves and ties reported by the commands are deterministic. The `moves` and `win` commands use the rule's move order; `win` selects the first winning move. Wythoff's game tries moves from pile 1, then pile 2, then both, with takes increasing.

## Exit codes

- `0`: the command completed successfully.
- `1`: a negative answer for `sum`, `win`, `period`, `misere`, or `wythoff` (the position is lost for the player to move, no winning move exists, or no period was found).
- `2`: an input or command error. The message is written to standard error with the `grundykit:` prefix; standard output is empty.

All program output lines end with LF on Windows, macOS and Linux.

## Layout

### Source

- `src/error.rs` - Defines the public game error type.
- `src/text.rs` - Provides display-safe text, labeled lines and integer-list formatting.
- `src/ints.rs` - Parses bounded decimal integers.
- `src/options.rs` - Parses command options and ordinary arguments.
- `src/io.rs` - Defines the I/O interface and standard-input word parsing.
- `src/rule.rs` - Parses game rules and formats their canonical names.
- `src/moves.rs` - Generates the ordered legal moves for a heap.
- `src/position.rs` - Parses and formats parts and positions.
- `src/nimber.rs` - Implements nim multiplication and inversion.
- `src/table.rs` - Computes mex values and Grundy tables.
- `src/solve.rs` - Evaluates positions and selects moves.
- `src/period.rs` - Detects periods in finite value sequences.
- `src/play.rs` - Plays positions to completion.
- `src/misere.rs` - Solves misere Nim positions.
- `src/wythoff.rs` - Computes Wythoff cold pairs and winning moves.
- `src/cmd_grundy.rs` - Implements the `grundy` command.
- `src/cmd_moves.rs` - Implements the `moves` command.
- `src/cmd_sum.rs` - Implements the `sum` command.
- `src/cmd_win.rs` - Implements the `win` command.
- `src/cmd_play.rs` - Implements the `play` command.
- `src/cmd_period.rs` - Implements the `period` command.
- `src/cmd_misere.rs` - Implements the `misere` command.
- `src/cmd_wythoff.rs` - Implements the `wythoff` command.
- `src/cmd_pairs.rs` - Implements the `pairs` command.
- `src/cmd_nimber.rs` - Implements the `nimber` command.
- `src/cli.rs` - Defines usage text and dispatches commands.
- `src/main.rs` - Connects the command-line process to the library.
- `src/lib.rs` - Exposes the library modules.

### Tests

- `tests/core_tests.rs` - Tests shared text, integer, option and input helpers.
- `tests/rule_tests.rs` - Tests rule parsing, moves and positions.
- `tests/nimber_tests.rs` - Tests nim arithmetic.
- `tests/table_tests.rs` - Tests Grundy tables and position solving.
- `tests/period_tests.rs` - Tests finite-sequence period detection.
- `tests/play_tests.rs` - Tests deterministic game transcripts.
- `tests/misere_tests.rs` - Tests misere Nim outcomes and moves.
- `tests/wythoff_tests.rs` - Tests cold pairs and winning moves.
- `tests/command_tests.rs` - Tests the ten command adapters.
- `tests/cli_tests.rs` - Tests command-line dispatch and process behavior.
- `tests/docs_tests.rs` - Tests the README usage, examples and file inventory.

## Development

Run the formatting, lint and test checks from the repository root:

```text
cargo fmt --check
cargo clippy -- -D warnings
cargo test
```

## License

MIT, see [LICENSE](LICENSE).

---

Built with BAKED
