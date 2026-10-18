use crate::error::GameError;
use crate::text::show;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Rule {
    Nim,
    Sub(Vec<u32>),
    Oct(Vec<u8>),
}

impl Rule {
    pub fn parse(text: &str) -> Result<Self, GameError> {
        if text == "nim" {
            return Ok(Self::Nim);
        }

        if let Some(takes_text) = text.strip_prefix("sub:") {
            let takes: Vec<&str> = takes_text.split(',').collect();
            if takes.iter().any(|take| !is_canonical_number(take)) {
                return Err(invalid_rule(text));
            }
            if takes.len() > 8 {
                return Err(GameError(format!(
                    "too many takes in {} (at most 8)",
                    show(text)
                )));
            }

            let mut values = Vec::with_capacity(takes.len());
            for take in takes {
                match take.parse::<u32>() {
                    Ok(value) if (1..=1000).contains(&value) => values.push(value),
                    _ => {
                        return Err(GameError(format!(
                            "invalid take {} in {} (expected 1 to 1000)",
                            show(take),
                            show(text)
                        )));
                    }
                }
            }
            values.sort_unstable();
            values.dedup();
            return Ok(Self::Sub(values));
        }

        if let Some(digits_text) = text.strip_prefix("oct:") {
            let digits = digits_text.as_bytes();
            if digits.is_empty() || !digits.iter().all(|digit| (b'0'..=b'7').contains(digit)) {
                return Err(invalid_rule(text));
            }
            if digits.len() > 12 {
                return Err(GameError(format!(
                    "too many digits in {} (at most 12)",
                    show(text)
                )));
            }
            if digits.last() == Some(&b'0') {
                return Err(GameError(format!(
                    "the last digit of {} must not be 0",
                    show(text)
                )));
            }
            return Ok(Self::Oct(digits.iter().map(|digit| digit - b'0').collect()));
        }

        Err(invalid_rule(text))
    }

    pub fn name(&self) -> String {
        match self {
            Self::Nim => "nim".to_string(),
            Self::Sub(takes) => format!(
                "sub:{}",
                takes
                    .iter()
                    .map(u32::to_string)
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::Oct(digits) => format!(
                "oct:{}",
                digits
                    .iter()
                    .map(u8::to_string)
                    .collect::<Vec<_>>()
                    .join("")
            ),
        }
    }
}

fn invalid_rule(text: &str) -> GameError {
    GameError(format!(
        "invalid rule {} (expected nim, sub:1,3,4 or oct:77)",
        show(text)
    ))
}

fn is_canonical_number(text: &str) -> bool {
    let bytes = text.as_bytes();
    text == "0"
        || (!bytes.is_empty()
            && (b'1'..=b'9').contains(&bytes[0])
            && bytes[1..].iter().all(u8::is_ascii_digit))
}
