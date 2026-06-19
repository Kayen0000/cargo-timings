use std::io::{self, Write};

use crate::{
    args::{ARG_SCHEME, ArgDef},
    consts::{BIN_NAME, DESCRIPTION},
};
pub fn print_err(msg: &str) {
    eprintln!("[ERROR]: {msg}");
}

pub fn print_warning(msg: &str) {
    eprintln!("[WARNING]: {msg}");
}

pub fn print_help<W: Write>(writer: W) -> io::Result<()> {
    let mut w = io::BufWriter::new(writer);

    writeln!(w, "Usage: {BIN_NAME} [OPTIONS] <PATH>\n",)?;

    writeln!(w, "{}\n", DESCRIPTION)?;

    writeln!(w, "{}", "Options:")?;

    let mut longest = (0, 0, 0, 0);
    for opt in ARG_SCHEME {
        let ArgDef { short, long, vname, desc } = *opt;
        if longest.0 < short.len() {
            longest.0 = short.len()
        }
        if longest.1 < long.len() {
            longest.1 = long.len()
        }
        if longest.2 < vname.len() {
            longest.2 = vname.len()
        }
        if longest.3 < desc.len() {
            longest.3 = desc.len()
        }
    }

    for opt in ARG_SCHEME {
        let (s, l, v, _) = longest;
        let width = l + v + 3;
        writeln!(w, "  {:<s$} {:<width$}{}", opt.short, format!("{}  {}", opt.long, opt.vname), opt.desc)?;
    }

    w.flush()
}
