use std::io::{self, Write};

use crate::{
    args::{Config, parse_args},
    timings::{Summary, parse_timings},
    utils::print_err,
};

mod args;
mod consts;
mod timings;
mod utils;

fn main() {
    let config = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            print_err(&e);
            std::process::exit(1)
        }
    };

    let summary = match parse_timings(&config.path) {
        Ok(v) => v,
        Err(e) => {
            print_err(&e);
            std::process::exit(1);
        }
    };

    print_summary(summary, &config).unwrap();
    std::process::exit(0)
}
/// EXPECTED OUTPUT
///
/// Targets: [targets]      Default
/// Profile: String         Full
/// Fresh units: num        Full
/// Dirty units: num        Full
/// Total units: num        Full
/// Max Concurency: String  Full
/// Build Start: String     Full
/// Total time: String      Default
/// rustc: [rustc]          Full
///
/// [DEFAULT.] [...EXTENDED...] [.FULL.]
/// UNIT TOTAL FRONTEND CODEGEN FEATURES
///
fn print_summary(mut summary: Summary, config: &Config) -> io::Result<()> {
    let mut w = io::BufWriter::new(io::stdout().lock());

    summary.timings.sort();
    //initial 1, 2, 3
    //reverse true 1, 2, 3
    //reverse false 3, 2 ,1
    //
    //top 1 && reverse false 3
    //top 1 && reverse true 1
    if !config.reverse {
        summary.timings.reverse();
    }

    if let Some(n) = config.top {
        let n = n as usize;
        summary.timings.truncate(n);
    }

    // unit, total, front, code
    let mut longest = (4, 5, 8, 7);
    for timing in &summary.timings {
        if longest.0 < timing.unit.len() {
            longest.0 = timing.unit.len()
        }
        if longest.1 < timing.total.len() {
            longest.1 = timing.total.len()
        }
        if longest.2 < timing.frontend.len() {
            longest.2 = timing.frontend.len()
        }
        if longest.3 < timing.codegen.len() {
            longest.3 = timing.codegen.len()
        }
    }
    let (unit, total, front, code) = longest;
    match config.detail_level {
        args::Detail::Default => {
            writeln!(w, "TARGETS: {:?}", summary.targets)?;
            writeln!(w, "TOTAL TIME: {}", summary.total_time)?;

            writeln!(w, "{:.<unit$} {:<total$}", "UNIT", "TOTAL")?;
            for timing in &summary.timings {
                writeln!(w, "{:<unit$} {:<total$}", timing.unit, timing.total)?;
            }
        }
        args::Detail::Extended => {
            writeln!(w, "TARGETS: {:?}", summary.targets)?;
            writeln!(w, "TOTAL TIME: {}", summary.total_time)?;

            writeln!(w, "{:.<unit$} {:<total$} {:<front$} {:<code$}", "UNIT", "TOTAL", "FRONTEND", "CODEGEN")?;
            for timing in &summary.timings {
                writeln!(
                    w,
                    "{:<unit$} {:<total$} {:<front$} {:<code$}",
                    timing.unit, timing.total, timing.frontend, timing.codegen
                )?;
            }
        }
        args::Detail::Full => {
            writeln!(w, "TARGETS: {:?}", summary.targets)?;
            writeln!(w, "PROFILE: {}", summary.profile)?;
            writeln!(w, "FRESH UNITS: {}", summary.fresh_units)?;
            writeln!(w, "DIRTY UNITS: {}", summary.dirty_units)?;
            writeln!(w, "TOTAL UNITS: {}", summary.total_units)?;
            writeln!(w, "MAX COUNCURENCY: {}", summary.max_concurency)?;
            writeln!(w, "BUILD START: {}", summary.build_start)?;
            writeln!(w, "TOTAL TIME: {}", summary.total_time)?;
            writeln!(w, "RUSTC: {:?}", summary.rustc)?;

            writeln!(
                w,
                "{:.<unit$} {:<total$} {:<front$} {:<code$} {}",
                "UNIT", "TOTAL", "FRONTEND", "CODEGEN", "FEATURES"
            )?;
            for timing in &summary.timings {
                writeln!(
                    w,
                    "{:.<unit$} {:<total$} {:<front$} {:<code$} {}",
                    timing.unit, timing.total, timing.frontend, timing.codegen, timing.features
                )?;
            }
        }
    }

    w.flush()
}
