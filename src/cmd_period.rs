use crate::error::GameError;
use crate::io::Io;
use crate::options::{self, Spec};
use crate::period;
use crate::rule::Rule;
use crate::table;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(
        argv,
        &[
            Spec {
                name: "--max",
                lo: 1,
                hi: 5000,
                fallback: 300,
            },
            Spec {
                name: "--cycles",
                lo: 2,
                hi: 10,
                fallback: 3,
            },
        ],
    )?;
    if parsed.args.is_empty() {
        return Err(GameError("missing RULE".to_owned()));
    }
    if parsed.args.len() > 1 {
        return Err(GameError("too many arguments".to_owned()));
    }

    let rule = Rule::parse(&parsed.args[0])?;
    let max = parsed.values["--max"] as u32;
    let cycles = parsed.values["--cycles"] as usize;
    let values = table::grundy(&rule, max);
    let mut lines = vec![
        text::line("rule", &rule.name()),
        text::line("max", &max.to_string()),
    ];
    let code = match period::detect_with(&values, cycles) {
        Some(found) => {
            lines.push(text::line("preperiod", &found.pre.to_string()));
            lines.push(text::line("period", &found.period.to_string()));
            lines.push(text::line(
                "cycle",
                &text::join(&values[found.pre..found.pre + found.period]),
            ));
            0
        }
        None => {
            lines.push(text::line("period", "none"));
            1
        }
    };
    let output = lines.join("\n") + "\n";

    io.out(&output);
    Ok(code)
}
