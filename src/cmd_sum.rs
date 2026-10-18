use crate::error::GameError;
use crate::io::{self, Io};
use crate::options;
use crate::position::Part;
use crate::rule::Rule;
use crate::table;
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

    let mut required = Vec::<(Rule, u32)>::new();
    let mut part_tables = Vec::with_capacity(parts.len());
    for part in &parts {
        let index = match required.iter().position(|(rule, _)| rule == &part.rule) {
            Some(index) => {
                required[index].1 = required[index].1.max(part.size);
                index
            }
            None => {
                required.push((part.rule.clone(), part.size));
                required.len() - 1
            }
        };
        part_tables.push(index);
    }

    let tables = required
        .into_iter()
        .map(|(rule, max_size)| table::grundy(&rule, max_size))
        .collect::<Vec<_>>();

    let mut total = 0_u64;
    let mut lines = vec![text::line("parts", &parts.len().to_string())];
    for (part, table_index) in parts.iter().zip(part_tables) {
        let value = tables[table_index][part.size as usize];
        total ^= value;
        lines.push(text::line(
            "part",
            &format!("{}  value {}", part.name(), value),
        ));
    }
    lines.push(text::line("value", &total.to_string()));
    lines.push(text::line(
        "winner",
        if total == 0 { "previous" } else { "next" },
    ));

    let output = lines.join("\n") + "\n";
    io.out(&output);
    Ok(if total == 0 { 1 } else { 0 })
}
