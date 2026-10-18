use crate::error::GameError;
use crate::io::Io;
use crate::options::{self, Spec};
use crate::rule::Rule;
use crate::table;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(
        argv,
        &[Spec {
            name: "--max",
            lo: 0,
            hi: 2000,
            fallback: 20,
        }],
    )?;
    if parsed.args.is_empty() {
        return Err(GameError("missing RULE".to_owned()));
    }
    if parsed.args.len() > 1 {
        return Err(GameError("too many arguments".to_owned()));
    }

    let rule = Rule::parse(&parsed.args[0])?;
    let max = parsed.values["--max"] as u32;
    let values = table::grundy(&rule, max);
    let zeros = values
        .iter()
        .enumerate()
        .filter_map(|(size, value)| (*value == 0).then_some(size as u64))
        .collect::<Vec<_>>();
    let largest = values.iter().copied().max().unwrap_or(0);
    let output = [
        text::line("rule", &rule.name()),
        text::line("max", &max.to_string()),
        text::line("values", &text::join(&values)),
        text::line("zeros", &text::join(&zeros)),
        text::line("largest", &largest.to_string()),
    ]
    .join("\n")
        + "\n";

    io.out(&output);
    Ok(0)
}
