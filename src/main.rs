use std::ffi::OsString;

use cargo_timings::{cli::output::print_summary, config::Config, error::print_err, parser::parse_timings};
use clap::Parser;

fn main() {
    let mut args: Vec<OsString> = std::env::args_os().collect();

    if args.len() > 1 && (args[1] == "timings" || args[1] == "cargo-timings") {
        args.remove(1);
    }

    let config = Config::parse_from(args);

    let summary = match parse_timings(&config.path) {
        Ok(v) => v,
        Err(e) => {
            print_err(&e.to_string());
            std::process::exit(1);
        }
    };
    #[cfg(feature = "tui")]
    if config.interactive {
        use cargo_timings::ui::renderer::run_tui_loop;

        if let Err(e) = run_tui_loop(summary, config) {
            print_err(&e.to_string());
            std::process::exit(1);
        }
        std::process::exit(0);
    }

    if let Err(e) = print_summary(summary, &config) {
        print_err(&e.to_string());
        std::process::exit(1);
    }
}
