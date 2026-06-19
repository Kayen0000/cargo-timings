# cargo-timings

Minimal binary to view slowest dependencies in your Rust project without leaving terminal.

## Installation

```bash
cargo install cargo-timings
````

## Usage

First, compile your Rust project using Cargo's built-in timing flag:

```bash
cargo build --timings
```

Then, run the tool inside your project directory:

```bash
# Automatically finds the standard target html path and lists bottlenecks
cargo timings

# Limit output to the top 5 slowest dependencies
cargo timings --top 5

# Get granular metrics including frontend and codegen stages
cargo timings --detail extended
```

Enjoy clean summary, without opening a browser:

```bash
TARGETS: ["cargo-timings 0.1.0 ( cargo-timings \"bin\")"]
TOTAL TIME: 103.6s (1m 43.6s)
UNIT.................................... TOTAL
regex-automata v0.4.14                   50.4s
regex-syntax v0.8.11                     48.3s
aho-corasick v1.1.4                      38.3s
regex v1.12.4                            10.1s
memchr v2.8.2                            7.5s 
cargo-timings v0.1.0 cargo-timings "bin" 7.0s
```

###### If ```cargo-timings``` optimized your workflow, consider dropping tip via [PayPal](https://paypal.me/dominikleszczynski0)
