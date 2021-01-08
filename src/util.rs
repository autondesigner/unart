use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_seeded_rng() -> StdRng {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    StdRng::seed_from_u64(since_the_epoch.as_millis() as u64)
}

pub fn iterate_fibn(fib: &mut Vec<usize>) {
    let length = fib.len();
    let mut next_fib = fib[0];
    for i in 1..length {
        next_fib = next_fib.wrapping_add(fib[i]);
    }
    for i in 0..length - 1 {
        fib[i] = fib[i + 1];
    }
    fib[length - 1] = next_fib;
}
