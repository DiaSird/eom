// Measure CPU time
use std::time::Instant;

pub fn time(start: Instant) {
    let end = start.elapsed();
    println!(
        "All completed in {}.{:03}s.",
        end.as_secs(),
        end.subsec_nanos() / 1_000_000
    );
}