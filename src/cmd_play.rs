use crate::error::GameError;
use crate::io::{self, Io};
use crate::options;
use crate::play;
use crate::position::{self, Part};
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
    let total_size = parts.iter().map(|part| part.size as u64).sum::<u64>();
    if total_size > 40 {
        return Err(GameError(format!(
            "total size {total_size} is above 40 (too large for play)"
        )));
    }

    let (turns, winner) = play::play(&parts);
    let mut lines = vec![text::line("start", &position::show(&parts))];
    for turn in &turns {
        lines.push(text::line(
            "move",
            &format!(
                "{}  {}  part {}  take {}  ->  {}",
                turn.number,
                turn.player,
                turn.step.part,
                turn.step.take,
                position::show(&turn.step.after)
            ),
        ));
    }
    lines.push(text::line("moves", &turns.len().to_string()));
    lines.push(text::line("winner", &winner.to_string()));

    let output = lines.join("\n") + "\n";
    io.out(&output);
    Ok(0)
}
