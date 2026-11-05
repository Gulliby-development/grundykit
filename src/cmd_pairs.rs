use crate::error::GameError;
use crate::io::Io;
use crate::options::{self, Spec};
use crate::text;
use crate::wythoff;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(
        argv,
        &[
            Spec {
                name: "--max",
                lo: 0,
                hi: 20_000,
                fallback: 20,
            },
            Spec {
                name: "--min",
                lo: 0,
                hi: 20_000,
                fallback: 0,
            },
        ],
    )?;

    if !parsed.args.is_empty() {
        return Err(GameError("too many arguments".to_owned()));
    }

    let max = parsed.values["--max"] as u32;
    let min = parsed.values["--min"] as u32;
    if min > max {
        return Err(GameError(format!("--min {min} is larger than --max {max}")));
    }

    let pairs: Vec<(u32, u32)> = wythoff::pairs(max)
        .into_iter()
        .filter(|(smaller, _)| *smaller >= min)
        .collect();
    let mut lines = Vec::with_capacity(pairs.len() + 3);
    if min > 0 {
        lines.push(text::line("min", &min.to_string()));
    }
    lines.push(text::line("max", &max.to_string()));
    lines.push(text::line("count", &pairs.len().to_string()));
    lines.extend(
        pairs
            .iter()
            .map(|(a, b)| text::line("pair", &format!("{a} {b}"))),
    );

    let output = format!("{}\n", lines.join("\n"));
    io.out(&output);
    Ok(0)
}
