use crate::moves::{self, Move};
use crate::position::Part;
use crate::rule::Rule;
use crate::table;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub part: usize,
    pub take: u32,
    pub leaves: Vec<u32>,
    pub after: Vec<Part>,
}

struct RuleTable {
    rule: Rule,
    values: Vec<u64>,
}

pub fn value(parts: &[Part]) -> u64 {
    let tables = prepare_tables(parts);
    position_value(parts, &tables)
}

pub fn winning(parts: &[Part]) -> Option<Step> {
    let tables = prepare_tables(parts);
    let total = position_value(parts, &tables);
    if total == 0 {
        return None;
    }

    for (index, part) in parts.iter().enumerate() {
        let old_value = value_at(&tables, &part.rule, part.size);
        for movement in moves::of(&part.rule, part.size) {
            let leaves_value = movement
                .leaves
                .iter()
                .map(|&size| value_at(&tables, &part.rule, size))
                .fold(0, |sum, leaf| sum ^ leaf);
            if total ^ old_value ^ leaves_value == 0 {
                return Some(make_step(parts, index, &movement));
            }
        }
    }
    None
}

pub fn first(parts: &[Part]) -> Option<Step> {
    for (index, part) in parts.iter().enumerate() {
        if let Some(movement) = moves::of(&part.rule, part.size).into_iter().next() {
            return Some(make_step(parts, index, &movement));
        }
    }
    None
}

fn prepare_tables(parts: &[Part]) -> Vec<RuleTable> {
    let mut required = Vec::<(Rule, u32)>::new();
    for part in parts {
        if let Some((_, max_size)) = required.iter_mut().find(|(rule, _)| rule == &part.rule) {
            *max_size = (*max_size).max(part.size);
        } else {
            required.push((part.rule.clone(), part.size));
        }
    }

    required
        .into_iter()
        .map(|(rule, max_size)| RuleTable {
            values: table::grundy(&rule, max_size),
            rule,
        })
        .collect()
}

fn position_value(parts: &[Part], tables: &[RuleTable]) -> u64 {
    parts
        .iter()
        .fold(0, |sum, part| sum ^ value_at(tables, &part.rule, part.size))
}

fn value_at(tables: &[RuleTable], rule: &Rule, size: u32) -> u64 {
    tables
        .iter()
        .find(|entry| entry.rule == *rule)
        .map(|entry| entry.values[size as usize])
        .unwrap_or(0)
}

fn make_step(parts: &[Part], index: usize, movement: &Move) -> Step {
    let mut after = Vec::with_capacity(parts.len() - 1 + movement.leaves.len());
    after.extend_from_slice(&parts[..index]);
    after.extend(movement.leaves.iter().map(|&size| Part {
        rule: parts[index].rule.clone(),
        size,
    }));
    after.extend_from_slice(&parts[index + 1..]);

    Step {
        part: index + 1,
        take: movement.take,
        leaves: movement.leaves.clone(),
        after,
    }
}
