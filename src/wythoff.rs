/// Returns the cold pairs whose smaller pile is at most `max`.
///
/// The supported analysis bound is 20,000. The bitmap is bounded by that
/// smaller-pile limit; partners larger than it are retained in the result but
/// do not need a bitmap entry.
pub fn pairs(max: u32) -> Vec<(u32, u32)> {
    generate_pairs(max)
}

/// Describes which pile or piles a Wythoff move takes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Where {
    Pile1,
    Pile2,
    Both,
}

/// Returns the first move to a cold pair, preserving the input pile order.
///
/// Candidate moves are checked in pile-1, pile-2, then both-piles order, with
/// increasing take sizes within each kind. Inputs in the command contract are
/// at most 20,000, so the partner table and all candidate scans are bounded.
pub fn winning(a: u32, b: u32) -> Option<(u32, Where, (u32, u32))> {
    let limit = a.max(b);
    let pairs = generate_pairs(limit);
    let mut partner = vec![None; limit as usize + 1];

    for (smaller, larger) in pairs {
        partner[smaller as usize] = Some(larger);
        if larger <= limit {
            partner[larger as usize] = Some(smaller);
        }
    }

    if is_cold(a, b, &partner) {
        return None;
    }

    for take in 1..=a {
        let remaining = (a - take, b);
        if is_cold(remaining.0, remaining.1, &partner) {
            return Some((take, Where::Pile1, remaining));
        }
    }

    for take in 1..=b {
        let remaining = (a, b - take);
        if is_cold(remaining.0, remaining.1, &partner) {
            return Some((take, Where::Pile2, remaining));
        }
    }

    for take in 1..=a.min(b) {
        let remaining = (a - take, b - take);
        if is_cold(remaining.0, remaining.1, &partner) {
            return Some((take, Where::Both, remaining));
        }
    }

    None
}

fn generate_pairs(max: u32) -> Vec<(u32, u32)> {
    let limit = max as usize;
    let mut used = vec![false; limit + 1];
    used[0] = true;

    let mut result = vec![(0, 0)];
    let mut difference = 1_u32;

    // A missing entry is the mex of all values used by earlier pairs.
    // If none is missing, the mex is above max and generation is complete.
    while let Some(smaller_index) = used.iter().position(|is_used| !is_used) {
        let smaller = smaller_index as u32;
        let larger = smaller + difference;

        result.push((smaller, larger));
        used[smaller_index] = true;
        if larger <= max {
            used[larger as usize] = true;
        }
        difference += 1;
    }

    result
}

fn is_cold(a: u32, b: u32, partner: &[Option<u32>]) -> bool {
    (a as usize) < partner.len() && partner[a as usize] == Some(b)
}
