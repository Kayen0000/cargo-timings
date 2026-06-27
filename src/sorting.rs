use crate::{
    config::{Config, Order},
    parser::Timing,
};

pub fn sort_timings(timings: &mut Vec<Timing>, config: &Config) {
    timings.retain(|t| t.total.ge(&std::time::Duration::from_secs_f32(config.min_time)));

    timings.retain(|t| t.unit.contains(&config.search));

    if let Some(n) = config.top {
        timings.truncate(n);
    }

    match config.order {
        Order::Ascending => timings.sort_by_key(|a| a.total),
        Order::Descending => timings.sort_by_key(|b| std::cmp::Reverse(b.total)),
    }
}
