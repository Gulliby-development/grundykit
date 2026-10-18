use crate::error::GameError;
use crate::ints;
use crate::rule::Rule;
use crate::text::show as show_text;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub rule: Rule,
    pub size: u32,
}

impl Part {
    pub fn parse(text: &str) -> Result<Self, GameError> {
        let Some((rule_text, size_text)) = text.split_once('/') else {
            return Err(GameError(format!(
                "invalid part {} (expected RULE/SIZE such as nim/5 or oct:77/9)",
                show_text(text)
            )));
        };
        let rule = Rule::parse(rule_text)?;
        let size = ints::parse("size", size_text, 0, 2000)? as u32;
        Ok(Self { rule, size })
    }

    pub fn name(&self) -> String {
        format!("{}/{}", self.rule.name(), self.size)
    }
}

pub fn show(parts: &[Part]) -> String {
    if parts.is_empty() {
        "(empty)".to_string()
    } else {
        parts.iter().map(Part::name).collect::<Vec<_>>().join(" ")
    }
}

pub fn leaves(heaps: &[u32]) -> String {
    if heaps.is_empty() {
        "none".to_string()
    } else {
        heaps
            .iter()
            .map(u32::to_string)
            .collect::<Vec<_>>()
            .join("+")
    }
}
