use crate::error::GameError;
use crate::text::show;

pub fn parse(name: &str, text: &str, lo: u64, hi: u64) -> Result<u64, GameError> {
    let bytes = text.as_bytes();
    let canonical = text == "0"
        || (!bytes.is_empty()
            && bytes.len() <= 10
            && (b'1'..=b'9').contains(&bytes[0])
            && bytes[1..].iter().all(u8::is_ascii_digit));

    let parsed = if canonical {
        text.parse::<u64>().ok()
    } else {
        None
    };

    match parsed {
        Some(value) if (lo..=hi).contains(&value) => Ok(value),
        _ => Err(GameError(format!(
            "invalid {name} {} (expected an integer from {lo} to {hi})",
            show(text)
        ))),
    }
}
