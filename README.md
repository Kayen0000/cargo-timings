# cargo-timings

Terminal-based visualizer for `cargo-timing.html` files, built entirely in Rust.

[<img alt="crates.io" src="https://img.shields.io/crates/v/cargo-timings.svg?style=for-the-badge&color=fc8d61&logo=rust" height="20">](https://crates.io/crates/cargo-timings)

## Installation

```bash
# Standard installation
cargo install cargo-timings

# With tui interactive mode
cargo install cargo-timings --features tui
```

## Usage

First, compile your Rust project using Cargo's built-in timing flag:

```bash
cargo build --timings
```

Then, run the tool inside your project directory:

```bash
# Automatically finds the standard target html path and lists bottlenecks
cargo timings

# Find matching dependencies
cargo timings --search "build-script"

# Get granular metrics including frontend and codegen stages
cargo timings --detail extended
cargo timings --detail full

# View in interactive mode
cargo timings -i
```

Enjoy clean summary, without opening a browser:

```bash
TARGETS: ["cargo-timings 0.2.0 (lib)", "cargo-timings 0.2.0 ( cargo-timings \"bin\")"]
TOTAL TIME: 215.1s
UNIT.................. TOTAL
clap_builder v4.6.0    50.9s
scraper v0.27.0        31.7s
syn v2.0.118           24.4s
ratatui-widgets v0.3.2 22.4s
ratatui-core v0.1.2    21.9s
```

## Support
You can support this project via [PayPal](https://paypal.me/dominikleszczynski0)
