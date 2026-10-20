/// A detected period in a finite sequence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Period {
    pub pre: usize,
    pub period: usize,
}

/// Finds the smallest period supported by at least three complete periods.
pub fn detect(values: &[u64]) -> Option<Period> {
    detect_with(values, 3)
}

/// Finds the smallest period supported by at least `cycles` complete periods.
pub fn detect_with(values: &[u64], cycles: usize) -> Option<Period> {
    let len = values.len();

    for period in 1..=len / cycles {
        let mut last_mismatch = None;
        for index in 0..len - period {
            if values[index] != values[index + period] {
                last_mismatch = Some(index);
            }
        }

        let pre = last_mismatch.map_or(0, |index| index + 1);
        if pre <= len - cycles * period {
            return Some(Period { pre, period });
        }
    }

    None
}
