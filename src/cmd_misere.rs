use crate::error::GameError;
use crate::ints;
use crate::io::{self, Io};
use crate::misere;
use crate::options;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(argv, &[])?;
    let args = io::words(io, &parsed.args)?;

    if args.is_empty() {
        return Err(GameError("missing HEAP".to_owned()));
    }
    if args.len() > 8 {
        return Err(GameError("too many heaps (at most 8)".to_owned()));
    }

    let heaps = args
        .iter()
        .map(|heap| ints::parse("HEAP", heap, 0, 1000).map(|value| value as u32))
        .collect::<Result<Vec<_>, _>>()?;

    let big = heaps.iter().filter(|&&heap| heap >= 2).count();
    let nim_sum = heaps.iter().fold(0_u32, |sum, &heap| sum ^ heap);
    let previous_wins = misere::previous_wins(&heaps);

    let mut lines = vec![
        text::line(
            "heaps",
            &text::join(
                &heaps
                    .iter()
                    .map(|&heap| u64::from(heap))
                    .collect::<Vec<_>>(),
            ),
        ),
        text::line("big", &big.to_string()),
        text::line("nim-sum", &nim_sum.to_string()),
        text::line("winner", if previous_wins { "previous" } else { "next" }),
    ];

    let result = if previous_wins {
        lines.push(text::line("status", "no winning move"));
        1
    } else if let Some((heap, take)) = misere::winning(&heaps) {
        lines.push(text::line(
            "move",
            &format!(
                "heap {heap}  take {take}  leaves {}",
                heaps[heap - 1] - take
            ),
        ));
        0
    } else {
        lines.push(text::line("move", "none"));
        0
    };

    let output = format!("{}\n", lines.join("\n"));
    io.out(&output);
    Ok(result)
}
