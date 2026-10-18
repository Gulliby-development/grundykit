pub fn show(text: &str) -> String {
    let mut shown = String::with_capacity(text.len() + 2);
    shown.push('"');
    for ch in text.chars() {
        if ('!'..='~').contains(&ch) {
            shown.push(ch);
        } else {
            shown.push('?');
        }
    }
    shown.push('"');
    shown
}

pub fn line(key: &str, value: &str) -> String {
    let padding = 10usize.saturating_sub(key.chars().count());
    format!("{key}{}{value}", " ".repeat(padding + 2))
}

pub fn join(values: &[u64]) -> String {
    values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}
