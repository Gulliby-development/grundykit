use crate::rule::Rule;

/// Returns the smallest nonnegative value absent from `values`.
///
/// The marker vector has only `values.len() + 1` entries: by the pigeonhole
/// principle the mex cannot exceed the number of supplied values. Large
/// values are ignored before conversion or indexing.
pub fn mex(values: &[u64]) -> u64 {
    let mut present = vec![false; values.len() + 1];
    for &value in values {
        if let Ok(index) = usize::try_from(value)
            && index < present.len()
        {
            present[index] = true;
        }
    }

    present
        .iter()
        .position(|is_present| !is_present)
        .unwrap_or(present.len()) as u64
}

/// Returns the Grundy values for heap sizes zero through `max`, inclusive.
///
/// Nim is direct. For subtraction and octal games, values are marked with a
/// heap-specific stamp so the bounded scratch storage is reused without
/// clearing or allocating it for every heap. In an octal game each digit
/// contributes at most one no-heap/single-heap outcome and at most
/// `max / 2` split outcomes, so the scratch bound is linear in the maximum
/// heap size and the (at-most-12) digits. Values outside that bound cannot
/// change a mex: it is at most the number of legal moves.
pub fn grundy(rule: &Rule, max: u32) -> Vec<u64> {
    if matches!(rule, Rule::Nim) {
        return (0..=max).map(u64::from).collect();
    }

    let marking_capacity = match rule {
        Rule::Nim => unreachable!("Nim returned above"),
        Rule::Sub(takes) => takes.len().saturating_add(1),
        Rule::Oct(digits) => {
            let splits_per_digit = usize::try_from(max / 2).unwrap_or(usize::MAX);
            digits
                .len()
                .saturating_mul(splits_per_digit.saturating_add(1))
                .saturating_add(1)
        }
    };
    let mut marks = vec![0_u64; marking_capacity];
    let mut values = Vec::new();

    for heap in 0..=max {
        let generation = u64::from(heap) + 1;
        match rule {
            Rule::Nim => unreachable!("Nim returned above"),
            Rule::Sub(takes) => {
                for &take in takes {
                    if take <= heap {
                        mark(&mut marks, generation, values[(heap - take) as usize]);
                    }
                }
            }
            Rule::Oct(digits) => {
                for (index, &digit) in digits.iter().enumerate() {
                    let take = index as u32 + 1;
                    if take > heap {
                        break;
                    }
                    let remainder = heap - take;

                    if digit & 1 != 0 && remainder == 0 {
                        mark(&mut marks, generation, 0);
                    }
                    if digit & 2 != 0 && remainder >= 1 {
                        mark(&mut marks, generation, values[remainder as usize]);
                    }
                    if digit & 4 != 0 && remainder >= 2 {
                        for smaller in 1..=remainder / 2 {
                            let larger = remainder - smaller;
                            mark(
                                &mut marks,
                                generation,
                                values[smaller as usize] ^ values[larger as usize],
                            );
                        }
                    }
                }
            }
        }

        values.push(first_unmarked(&marks, generation));
    }

    values
}

fn mark(marks: &mut [u64], generation: u64, value: u64) {
    if let Ok(index) = usize::try_from(value)
        && let Some(slot) = marks.get_mut(index)
    {
        *slot = generation;
    }
}

fn first_unmarked(marks: &[u64], generation: u64) -> u64 {
    marks
        .iter()
        .position(|&mark| mark != generation)
        .unwrap_or(marks.len()) as u64
}
