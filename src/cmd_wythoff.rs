use crate::error::GameError;
use crate::ints;
use crate::io::Io;
use crate::options;
use crate::text;
use crate::wythoff::{self, Where};

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(argv, &[])?;
    let args = parsed.args;

    if args.is_empty() {
        return Err(GameError("missing A".to_owned()));
    }
    if args.len() == 1 {
        return Err(GameError("missing B".to_owned()));
    }
    if args.len() > 2 {
        return Err(GameError("too many arguments".to_owned()));
    }

    let a = ints::parse("A", &args[0], 0, 20_000)? as u32;
    let b = ints::parse("B", &args[1], 0, 20_000)? as u32;
    let mut lines = vec![text::line("piles", &format!("{a} {b}"))];

    let result = match wythoff::winning(a, b) {
        None => {
            lines.push(text::line("winner", "previous"));
            1
        }
        Some((take, where_from, (left, right))) => {
            let where_text = match where_from {
                Where::Pile1 => "pile 1",
                Where::Pile2 => "pile 2",
                Where::Both => "both",
            };
            lines.push(text::line("winner", "next"));
            lines.push(text::line(
                "move",
                &format!("take {take} from {where_text}"),
            ));
            lines.push(text::line("leaves", &format!("{left} {right}")));
            0
        }
    };

    let output = format!("{}\n", lines.join("\n"));
    io.out(&output);
    Ok(result)
}
