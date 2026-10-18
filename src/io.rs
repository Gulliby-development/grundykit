use crate::error::GameError;

pub trait Io {
    fn read_stdin(&self) -> Vec<u8>;
    fn out(&self, text: &str);
    fn err(&self, text: &str);
}

pub fn words(io: &dyn Io, args: &[String]) -> Result<Vec<String>, GameError> {
    if args.len() != 1 || args[0] != "-" {
        return Ok(args.to_vec());
    }

    let bytes = io.read_stdin();
    let input = std::str::from_utf8(&bytes)
        .map_err(|_| GameError("<stdin>: input is not valid UTF-8".to_owned()))?;
    let input = input.strip_prefix('\u{feff}').unwrap_or(input);
    let words: Vec<String> = input
        .split([' ', '\t', '\r', '\n'])
        .filter(|word| !word.is_empty())
        .map(str::to_owned)
        .collect();

    if words.is_empty() {
        Err(GameError("<stdin>: no input".to_owned()))
    } else {
        Ok(words)
    }
}
