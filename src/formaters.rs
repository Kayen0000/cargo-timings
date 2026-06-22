pub fn fmt_dur(dur: std::time::Duration) -> String {
    format!("{:.1}s", dur.as_secs_f32())
}
