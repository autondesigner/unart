use rand::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn time_seeded_rng() -> StdRng {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    StdRng::seed_from_u64(since_the_epoch.as_millis() as u64)
}
