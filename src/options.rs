use crate::error::GameError;
use crate::ints;
use crate::text::show;
use std::collections::BTreeMap;

pub struct Spec {
    pub name: &'static str,
    pub lo: u64,
    pub hi: u64,
    pub fallback: u64,
}

pub struct Parsed {
    pub values: BTreeMap<String, u64>,
    pub args: Vec<String>,
}

pub fn parse(argv: &[String], specs: &[Spec]) -> Result<Parsed, GameError> {
    let mut raw_values = BTreeMap::<String, String>::new();
    let mut args = Vec::new();
    let mut index = 0;

    while index < argv.len() {
        let argument = &argv[index];
        if !argument.starts_with("--") {
            args.push(argument.clone());
            index += 1;
            continue;
        }

        let (name, inline_value) = match argument.split_once('=') {
            Some((name, value)) => (name, Some(value.to_owned())),
            None => (argument.as_str(), None),
        };

        if !specs.iter().any(|spec| spec.name == name) {
            return Err(GameError(format!("unknown option {}", show(argument))));
        }

        let value = match inline_value {
            Some(value) => value,
            None => {
                index += 1;
                match argv.get(index) {
                    Some(value) => value.clone(),
                    None => {
                        return Err(GameError(format!("{name} needs a value")));
                    }
                }
            }
        };
        raw_values.insert(name.to_owned(), value);
        index += 1;
    }

    let mut values = BTreeMap::new();
    for spec in specs {
        let value = match raw_values.get(spec.name) {
            Some(raw) => ints::parse(spec.name, raw, spec.lo, spec.hi)?,
            None => spec.fallback,
        };
        values.insert(spec.name.to_owned(), value);
    }

    Ok(Parsed { values, args })
}
