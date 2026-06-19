use std::{fs, path::PathBuf};

#[derive(Debug)]
pub struct Summary {
    pub targets: Vec<String>,
    pub profile: String,
    pub fresh_units: String,
    pub dirty_units: String,
    pub total_units: String,
    pub max_concurency: String,
    pub build_start: String,
    pub total_time: String,
    pub rustc: Vec<String>,
    pub timings: Vec<Timing>,
}

#[derive(Debug, PartialEq, Ord, Eq)]
pub struct Timing {
    pub id: String,
    pub unit: String,
    pub total: String,
    pub frontend: String,
    pub codegen: String,
    pub features: String,
}

impl Timing {
    pub fn total_as_f32(&self) -> f32 {
        self.total.strip_suffix("s").unwrap_or("0").parse::<f32>().unwrap()
    }
}

impl PartialOrd for Timing {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.total.is_empty() || other.total.is_empty() {
            return None;
        }
        let self_total = self.total_as_f32();
        let other_total = other.total_as_f32();
        if self_total < other_total {
            return Some(std::cmp::Ordering::Less);
        }
        if self_total > other_total {
            return Some(std::cmp::Ordering::Greater);
        }
        if self_total == other_total {
            return Some(std::cmp::Ordering::Equal);
        }
        None
    }
}
pub fn parse_timings(path: &PathBuf) -> Result<Summary, String> {
    if !path.exists() {
        return Err(format!(
            "{} does not exists. Make sure to run 'cargo build --timings' first",
            path.display()
        ));
    }
    let html = fs::read_to_string(path).map_err(|e| format!("failed to read {}: {e}", path.display()))?;
    let re = regex::Regex::new(r"<table[\s\S]*?<\/table>").unwrap();

    // 0:Summary, 1:input (don't care) 2:details
    let tables = re.find_iter(&html).map(|s| s.as_str()).collect::<Vec<&str>>();

    if tables.len() < 3 {
        return Err("Provided file is not a valid cargo-timing HTML report.".to_string());
    }

    let re = regex::Regex::new(r"<td>(.*?)<\/td>").unwrap();

    // 0:name (don't care), 1:value
    let mut cells = re
        .captures_iter(tables[0])
        .enumerate()
        .filter(|(i, _)| i & 1 == 1)
        .map(|(_, c)| c[1].to_string());

    let br_re = regex::Regex::new(r"<br>").unwrap();

    let target_str = cells.next().unwrap_or_default();
    let targets = br_re.split(&target_str).map(|s| s.to_string()).collect::<Vec<String>>();

    let profile = cells.next().unwrap_or_default().to_string();
    let fresh_units = cells.next().unwrap_or_default();
    let dirty_units = cells.next().unwrap_or_default();
    let total_units = cells.next().unwrap_or_default();
    let max_concurency = cells.next().unwrap_or_default();
    let build_start = cells.next().unwrap_or_default();
    let total_time = cells.next().unwrap_or_default();

    let rustc_str = cells.next().unwrap_or_default();
    let rustc = br_re.split(&rustc_str).map(|s| s.to_string()).collect::<Vec<String>>();

    let mut summary = Summary {
        targets,
        profile,
        fresh_units,
        dirty_units,
        total_units,
        max_concurency,
        build_start,
        total_time,
        rustc,
        timings: Vec::new(),
    };

    let re = regex::Regex::new(r"<tr>[\s\S]*?<\/tr>").unwrap();
    let timings_str = re.find_iter(tables[2]).map(|s| s.as_str()).skip(1);

    let re = regex::Regex::new(r"<td>(.*?)<\/td>").unwrap();
    for str in timings_str {
        // 0:n, 1:unit, 2:total, 3:frontend, 4:codegen, 5:features
        let mut val = re.captures_iter(str).map(|s| s[1].to_string());
        let t = Timing {
            id: val.next().unwrap_or("<NOT FOUND>".to_string()),
            unit: val.next().unwrap_or("<NOT FOUND>".to_string()),
            total: val.next().unwrap_or("<NOT FOUND>".to_string()),
            frontend: val.next().unwrap_or("<NOT FOUND>".to_string()),
            codegen: val.next().unwrap_or("<NOT FOUND>".to_string()),
            features: val.next().unwrap_or("<NOT FOUND>".to_string()),
        };
        summary.timings.push(t);
    }
    Ok(summary)
}
