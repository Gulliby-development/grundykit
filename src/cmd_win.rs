use crate::error::GameError;
use crate::io::{self, Io};
use crate::options;
use crate::position::{self, Part};
use crate::solve;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(argv, &[])?;
    let words = io::words(io, &parsed.args)?;
    if words.is_empty() {
        return Err(GameError("missing PART".to_owned()));
    }
    if words.len() > 8 {
        return Err(GameError("too many parts (at most 8)".to_owned()));
    }

    let parts = words
        .iter()
        .map(|word| Part::parse(word))
        .collect::<Result<Vec<_>, _>>()?;
    let value = solve::value(&parts);
    let mut lines = vec![text::line("value", &value.to_string())];
    let code = match solve::winning(&parts) {
        Some(step) => {
            lines.push(text::line("part", &step.part.to_string()));
            lines.push(text::line("take", &step.take.to_string()));
            lines.push(text::line("leaves", &position::leaves(&step.leaves)));
            lines.push(text::line("after", &position::show(&step.after)));
            0
        }
        None => {
            lines.push(text::line("status", "no winning move"));
            1
        }
    };

    let output = lines.join("\n") + "\n";
    io.out(&output);
    Ok(code)
}
