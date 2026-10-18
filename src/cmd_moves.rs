use crate::error::GameError;
use crate::io::Io;
use crate::moves;
use crate::options;
use crate::position::{self, Part};
use crate::table;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(argv, &[])?;
    if parsed.args.is_empty() {
        return Err(GameError("missing PART".to_owned()));
    }
    if parsed.args.len() > 1 {
        return Err(GameError("too many arguments".to_owned()));
    }

    let part = Part::parse(&parsed.args[0])?;
    let values = table::grundy(&part.rule, part.size);
    let value = values[part.size as usize];
    let legal_moves = moves::of(&part.rule, part.size);
    let mut lines = vec![
        text::line("part", &part.name()),
        text::line("value", &value.to_string()),
        text::line("moves", &legal_moves.len().to_string()),
    ];

    for legal_move in legal_moves {
        let move_value = legal_move
            .leaves
            .iter()
            .fold(0_u64, |sum, &heap| sum ^ values[heap as usize]);
        lines.push(text::line(
            "move",
            &format!(
                "take {}  leaves {}  value {}",
                legal_move.take,
                position::leaves(&legal_move.leaves),
                move_value
            ),
        ));
    }
    let output = lines.join("\n") + "\n";

    io.out(&output);
    Ok(0)
}
