use std::{fs, path::PathBuf, time::Duration};

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use scraper::{Html, Selector};

#[derive(Debug)]
pub struct Summary {
    pub targets: Vec<String>,
    pub profile: String,
    pub fresh_units: u32,
    pub dirty_units: u32,
    pub total_units: u32,
    pub max_concurency: u32,
    pub concurency_details: String,
    pub build_start: DateTime<Utc>,
    pub total_time: Duration,
    pub rustc: Vec<String>,
    pub timings: Vec<Timing>,
}

#[derive(Debug, Clone)]
pub struct Timing {
    pub id: u32,
    pub unit: String,
    pub total: Duration,
    pub frontend: Option<Duration>,
    pub codegen: Option<Duration>,
    pub features: Vec<String>,
}

pub fn parse_timings(path: &PathBuf) -> Result<Summary> {
    if !path.exists() {
        anyhow::bail!(
            "Could not find timing report at '{}'.\n\nRun 'cargo build --timings' to generate this file",
            path.display()
        );
    }

    let html_content = fs::read_to_string(path).with_context(|| format!("Failed to process timing report at '{}'", path.display()))?;

    let document = Html::parse_document(&html_content);

    let table_selector = Selector::parse("table").unwrap();
    let row_selector = Selector::parse("tr").unwrap();
    let cell_selector = Selector::parse("td").unwrap();

    let tables: Vec<_> = document.select(&table_selector).collect();
    if tables.len() < 3 {
        anyhow::bail!("Provided invalid timings report: Expected at least 3 tables, found {}", tables.len());
    }

    // --- 1. PARSE SUMMARY TABLE (Table 0) ---
    let summary_cells: Vec<String> = tables[0]
        .select(&row_selector)
        .flat_map(|row| row.select(&cell_selector))
        .enumerate()
        .filter_map(|(i, cell)| {
            if i & 1 == 1 {
                Some(cell.text().collect::<Vec<_>>().join("\n"))
            } else {
                None
            }
        })
        .collect();

    if summary_cells.len() < 9 {
        anyhow::bail!("Summary table has insufficient data fields.");
    }

    let mut values = summary_cells.into_iter();
    let targets = values.next().unwrap_or_default().lines().map(|s| s.to_string()).collect();
    let profile = values.next().unwrap_or_default();
    let fresh_units = values.next().unwrap_or_default().parse().unwrap_or_default();
    let dirty_units = values.next().unwrap_or_default().parse().unwrap_or_default();
    let total_units = values.next().unwrap_or_default().parse().unwrap_or_default();

    let concurrency_raw = values.next().unwrap_or_default();
    let (max_concur_str, concurrency_details) = concurrency_raw.split_once(' ').unwrap_or((concurrency_raw.as_str(), ""));
    let max_concurrency = max_concur_str.parse().unwrap_or_default();

    let build_start = DateTime::parse_from_rfc3339(&values.next().unwrap_or_default())
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let total_time_str = values.next().unwrap_or_default();
    let total_time_clean = total_time_str.split_once(' ').unwrap_or(("0s", "")).0.trim_end_matches('s');
    let total_time = Duration::from_secs_f32(total_time_clean.parse().unwrap_or_default());

    let rustc = values.next().unwrap_or_default().lines().map(|s| s.to_string()).collect();
    let mut summary = Summary {
        targets,
        profile,
        fresh_units,
        dirty_units,
        total_units,
        max_concurency: max_concurrency,
        concurency_details: concurrency_details.to_string(),
        build_start,
        total_time,
        rustc,
        timings: Vec::new(),
    };

    // --- 2. PARSE TIMINGS TABLE (Table 2) ---
    for row in tables[2].select(&row_selector) {
        let cells: Vec<String> = row
            .select(&cell_selector)
            .map(|cell| cell.text().collect::<String>().trim().to_string())
            .collect();

        if cells.is_empty() || cells[0].ends_with("Unit") {
            continue;
        }

        let id = cells
            .first()
            .cloned()
            .unwrap_or_default()
            .trim_end_matches('.')
            .parse()
            .unwrap_or_default();
        let unit = cells.get(1).cloned().unwrap_or_default();

        let total = match cells.get(2) {
            Some(s) => Duration::from_secs_f32(parse_time_into_f32(s)?),
            None => continue,
        };

        let frontend = cells
            .get(3)
            .filter(|s| !s.is_empty())
            .and_then(|s| parse_time_into_f32(s).ok())
            .map(Duration::from_secs_f32);

        let codegen = cells
            .get(4)
            .filter(|s| !s.is_empty())
            .and_then(|s| parse_time_into_f32(s).ok())
            .map(Duration::from_secs_f32);

        let features = cells
            .get(5)
            .map(|s| s.split(",").map(|f| f.trim().to_string()).collect())
            .unwrap_or_default();

        summary.timings.push(Timing {
            id,
            unit,
            total,
            frontend,
            codegen,
            features,
        });
    }

    Ok(summary)
}

fn parse_time_into_f32(s: &str) -> Result<f32> {
    let clean = s.split_once('s').with_context(|| format!("invalid value {s}"))?.0;
    clean.parse::<f32>().with_context(|| format!("could not parse {clean} into f32"))
}
