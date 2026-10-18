use crate::error::GameError;
use crate::ints;
use crate::io::Io;
use crate::nimber;
use crate::options;
use crate::text;

pub fn run(argv: &[String], io: &dyn Io) -> Result<i32, GameError> {
    let parsed = options::parse(argv, &[])?;

    if parsed.args.is_empty() {
        return Err(GameError("missing A".to_owned()));
    }
    if parsed.args.len() == 1 {
        return Err(GameError("missing B".to_owned()));
    }
    if parsed.args.len() > 2 {
        return Err(GameError("too many arguments".to_owned()));
    }

    let a = ints::parse("A", &parsed.args[0], 0, u32::MAX as u64)? as u32;
    let b = ints::parse("B", &parsed.args[1], 0, u32::MAX as u64)? as u32;
    let sum = a ^ b;
    let product = nimber::product(a, b);
    let inverse_a = nimber::inverse(a)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "none".to_owned());
    let inverse_b = nimber::inverse(b)
        .map(|value| value.to_string())
        .unwrap_or_else(|| "none".to_owned());

    let lines = [
        text::line("a", &a.to_string()),
        text::line("b", &b.to_string()),
        text::line("sum", &sum.to_string()),
        text::line("product", &product.to_string()),
        text::line("inverse-a", &inverse_a),
        text::line("inverse-b", &inverse_b),
    ];
    let output = format!("{}\n", lines.join("\n"));
    io.out(&output);
    Ok(0)
}
