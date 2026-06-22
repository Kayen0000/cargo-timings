use io::Write;
use std::io;

use crate::{
    config::{Config, Detail, Order},
    formaters::fmt_dur,
    parser::Summary,
};

/// EXPECTED OUTPUT
///
/// Targets: [targets]      Default
/// Profile: String         Full
/// Fresh units: num        Full
/// Dirty units: num        Full
/// Total units: num        Full
/// Max Concurency: String  Full
/// Build Start: DateTime   Full
/// Total time: Duration    Default
/// rustc: [rustc]          Full
///
/// [DEFAULT.] [...EXTENDED...] [.FULL.]
/// UNIT TOTAL FRONTEND CODEGEN FEATURES
pub fn print_summary(mut summary: Summary, config: &Config) -> io::Result<()> {
    let mut w = io::BufWriter::new(io::stdout().lock());

    match config.order {
        Order::Ascending => summary.timings.sort_by_key(|a| a.total),
        Order::Descending => summary.timings.sort_by_key(|b| std::cmp::Reverse(b.total)),
    }

    if let Some(sterm) = &config.search {
        summary.timings.retain(|t| t.unit.contains(sterm));
    }

    if let Some(n) = config.top {
        summary.timings.truncate(n);
    }

    let mut longest = (4, 5, 8, 7);
    for timing in &summary.timings {
        let total_str = fmt_dur(timing.total);
        let frontend_str = timing.frontend.map(fmt_dur).unwrap_or_default();
        let codegen_str = timing.codegen.map(fmt_dur).unwrap_or_default();

        longest.0 = longest.0.max(timing.unit.len());
        longest.1 = longest.1.max(total_str.len());
        longest.2 = longest.2.max(frontend_str.len());
        longest.3 = longest.3.max(codegen_str.len());
    }
    let (unit, total, front, code) = longest;
    match config.detail_level {
        Detail::Default => {
            writeln!(w, "TARGETS: {:?}", summary.targets)?;
            writeln!(w, "TOTAL TIME: {}", fmt_dur(summary.total_time))?;

            writeln!(w, "{:.<unit$} {:<total$}", "UNIT", "TOTAL")?;
            for timing in &summary.timings {
                writeln!(w, "{:<unit$} {:<total$}", timing.unit, fmt_dur(timing.total))?;
            }
        }
        Detail::Extended => {
            writeln!(w, "TARGETS: {:?}", summary.targets)?;
            writeln!(w, "TOTAL TIME: {}", fmt_dur(summary.total_time))?;

            writeln!(w, "{:.<unit$} {:<total$} {:<front$} {:<code$}", "UNIT", "TOTAL", "FRONTEND", "CODEGEN")?;
            for timing in &summary.timings {
                writeln!(
                    w,
                    "{:<unit$} {:<total$} {:<front$} {:<code$}",
                    timing.unit,
                    fmt_dur(timing.total),
                    timing.frontend.map(fmt_dur).unwrap_or_default(),
                    timing.codegen.map(fmt_dur).unwrap_or_default()
                )?;
            }
        }
        Detail::Full => {
            writeln!(w, "TARGETS: {:?}", summary.targets)?;
            writeln!(w, "PROFILE: {}", summary.profile)?;
            writeln!(w, "FRESH UNITS: {}", summary.fresh_units)?;
            writeln!(w, "DIRTY UNITS: {}", summary.dirty_units)?;
            writeln!(w, "TOTAL UNITS: {}", summary.total_units)?;
            writeln!(w, "MAX COUNCURENCY: {}", summary.max_concurency)?;
            writeln!(w, "CONCURENCY DETAIS: {}", summary.concurency_details)?;
            writeln!(w, "BUILD START: {}", summary.build_start)?;
            writeln!(w, "TOTAL TIME: {}", fmt_dur(summary.total_time))?;
            writeln!(w, "RUSTC: {:?}", summary.rustc)?;

            writeln!(
                w,
                "{:.<unit$} {:<total$} {:<front$} {:<code$} FEATURES",
                "UNIT", "TOTAL", "FRONTEND", "CODEGEN"
            )?;
            for timing in &summary.timings {
                writeln!(
                    w,
                    "{:<unit$} {:<total$} {:<front$} {:<code$} {:?}",
                    timing.unit,
                    fmt_dur(timing.total),
                    timing.frontend.map(fmt_dur).unwrap_or_default(),
                    timing.codegen.map(fmt_dur).unwrap_or_default(),
                    timing.features
                )?;
            }
        }
    }

    w.flush()
}
