/// Returns whether the player who just moved wins under misere Nim.
pub fn previous_wins(heaps: &[u32]) -> bool {
    let mut singletons = 0_usize;
    let mut big_heaps = 0_usize;
    let mut nim_sum = 0_u32;

    for &heap in heaps {
        if heap == 1 {
            singletons += 1;
        } else if heap >= 2 {
            big_heaps += 1;
        }
        nim_sum ^= heap;
    }

    position_previous_wins(singletons, big_heaps, nim_sum)
}

/// Returns the first move, ordered by heap and then by increasing take, that
/// leaves a position which is losing for the next player.
pub fn winning(heaps: &[u32]) -> Option<(usize, u32)> {
    let mut singletons = 0_usize;
    let mut big_heaps = 0_usize;
    let mut nim_sum = 0_u32;

    for &heap in heaps {
        if heap == 1 {
            singletons += 1;
        } else if heap >= 2 {
            big_heaps += 1;
        }
        nim_sum ^= heap;
    }

    if position_previous_wins(singletons, big_heaps, nim_sum) {
        return None;
    }

    for (index, &heap) in heaps.iter().enumerate() {
        for take in 1..=heap {
            let remaining = heap - take;
            let next_singletons = singletons - usize::from(heap == 1) + usize::from(remaining == 1);
            let next_big_heaps = big_heaps - usize::from(heap >= 2) + usize::from(remaining >= 2);
            let next_nim_sum = nim_sum ^ heap ^ remaining;

            if position_previous_wins(next_singletons, next_big_heaps, next_nim_sum) {
                return Some((index + 1, take));
            }
        }
    }

    None
}

fn position_previous_wins(singletons: usize, big_heaps: usize, nim_sum: u32) -> bool {
    if big_heaps == 0 {
        singletons % 2 == 1
    } else {
        nim_sum == 0
    }
}
