use clap::Parser;
use std::path::PathBuf;

use clap::ValueEnum;
#[derive(Debug, ValueEnum, Clone, Copy)]
pub enum Order {
    Ascending,
    Descending,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum Detail {
    Default,
    Extended,
    Full,
}

#[derive(Debug, Parser)]
#[command(author, about, version)]
pub struct Config {
    #[arg(help = "path to timings", default_value = "./target/cargo-timings/cargo-timing.html")]
    pub path: PathBuf,

    #[arg(short, long, help = "Show top results according to sorting order", value_name = "NUM")]
    pub top: Option<usize>,

    #[arg(short, long = "detail", help = "How much detail should summary show.")]
    #[arg(value_name = "LEVEL", hide_default_value = true, default_value = "default")]
    pub detail_level: Detail,

    #[arg(long, help = "Specify order of an output based on compilation time")]
    #[arg(default_value = "descending", value_name = "ORDER")]
    pub order: Order,

    #[arg(short, long, help = "Shows results matching search term", value_name = "TERM")]
    #[arg(default_value = "", hide_default_value = true)]
    pub search: String,

    #[arg(
        short,
        long,
        help = "Shows results that took at least [SEC] seconds to compile",
        value_name = "SEC",
        default_value = "0.0"
    )]
    pub min_time: f32,

    #[cfg(feature = "tui")]
    #[arg(short, long, help = "Shows output in interactive tui")]
    pub interactive: bool,
}

impl Order {
    pub fn next(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }

    pub fn toggle_next(&mut self) {
        *self = self.next();
    }

    pub fn prev(self) -> Self {
        match self {
            Self::Ascending => Self::Descending,
            Self::Descending => Self::Ascending,
        }
    }

    pub fn toggle_prev(&mut self) {
        *self = self.prev();
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Ascending => "Ascending",
            Self::Descending => "Descending",
        }
    }
}
