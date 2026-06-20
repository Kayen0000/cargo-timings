use std::{env, path::PathBuf, str::FromStr};

use crate::utils::print_help;

pub const ARG_SCHEME: &[ArgDef] = &[
    ArgDef::new("-h", "--help", "", "Print help."),
    ArgDef::new("-V", "--version", "", "Print version info."),
    ArgDef::new("", "--top", "<NUM>", "Show top slowest dependencies."),
    ArgDef::new("-r", "--reverse", "", "Reverse output into ascending order."),
    ArgDef::new("", "--detail", "<LEVEL>", "Amount of data in summary. values [default, extended, full]"),
];

pub struct ArgDef {
    pub short: &'static str,
    pub long: &'static str,
    pub vname: &'static str,
    pub desc: &'static str,
}

#[derive(Debug)]
pub struct Config {
    pub path: PathBuf,
    pub top: Option<u16>,
    pub detail_level: Detail,
    pub reverse: bool,
}

#[derive(Debug)]
pub enum Detail {
    Default,
    Extended,
    Full,
}

pub fn parse_args() -> Result<Config, String> {
    let args = env::args().collect::<Vec<String>>();

    // cases:
    // 0:cargo-timings 
    // 0:cargo 1:timings
    // 0:caller --- n:timings
    let arg_start = args.iter().position(|a| a == "timings" || a == "cargo-timings" ).ok_or_else(|| format!("invalid arguments"))? + 1;
    let mut args = args[arg_start..].into_iter();

    let mut help = false;
    let mut version = false;
    let mut path = PathBuf::new();
    let mut top = None;
    let mut detail_level = Detail::Default;
    let mut reverse = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => help = true,
            "-V" | "--version" => version = true,
            "-r" | "--reverse" => reverse = true,
            "--top" => {
                let s = args.next().ok_or_else(|| format!("'{arg}' requires a value"))?;
                top = Some(s.parse::<u16>().map_err(|_| format!("{arg}: invalid value '{s}'",))?);
            }
            "--detail" => {
                let s = args.next().ok_or_else(|| format!("'{arg}' requires a value"))?;
                detail_level = Detail::from_str(&s).map_err(|e| format!("{arg}: {e}"))?;
            }
            v if !v.starts_with("-") && path.is_empty() => path = PathBuf::from(v),
            _ => {
                return Err(format!("unknown argument '{arg}'. Use '--help' to see more",));
            }
        }
    }

    if help {
        print_help(std::io::stdout().lock()).unwrap();
        std::process::exit(0);
    }

    if version {
        use crate::consts::VERSION;
        println!("{VERSION}");
        std::process::exit(0);
    }

    if path.is_empty() {
        path = PathBuf::from("target/cargo-timings/cargo-timing.html");
    }
    Ok(Config {
        path,
        reverse,
        detail_level,
        top,
    })
}

impl FromStr for Detail {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "default" => Ok(Self::Default),
            "extended" => Ok(Self::Extended),
            "full" => Ok(Self::Full),
            _ => Err(format!("invalid value '{s}'")),
        }
    }
}

impl ArgDef {
    const fn new(short: &'static str, long: &'static str, vname: &'static str, desc: &'static str) -> Self {
        Self { short, long, vname, desc }
    }
}
