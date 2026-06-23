#[cfg(feature = "tui")]
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Row, Table},
};
use ratatui::{
    layout,
    style::{Style, Stylize},
    text::{Line, ToLine},
    widgets::Paragraph,
};

use crate::{
    config,
    formaters::fmt_dur,
    parser,
    ui::{app_state::AppState, input_handler::InputHandler},
};

pub fn run_tui_loop(summary: parser::Summary, config: config::Config) -> anyhow::Result<()> {
    let mut term = ratatui::init();
    let input_handler = InputHandler::new(std::time::Duration::from_millis(150));
    let mut state = AppState::init(summary, config);

    state.sort_timings();
    loop {
        if let Some(input) = input_handler.read_input() {
            state.process_input(input);
            if state.should_quit {
                break;
            }
            state.sort_timings();
        }
        term.draw(|frame| render(&state, frame))?;
    }

    ratatui::restore();
    Ok(())
}

fn render(state: &AppState, frame: &mut Frame) {
    let mut max_len = 10;
    for t in &state.summary.timings {
        if max_len < (t.unit.len() + fmt_dur(t.total).len()) {
            max_len = t.unit.len() + fmt_dur(t.total).len();
        }
    }

    let [up, down] = Layout::new(ratatui::layout::Direction::Vertical, [Constraint::Fill(1), Constraint::Fill(2)]).areas(frame.area());
    let [down_left, down_right] = Layout::new(
        ratatui::layout::Direction::Horizontal,
        [Constraint::Min(max_len as u16 + 6), Constraint::Percentage(100)],
    )
    .areas(down);

    render_summary(state, up, frame);
    render_timings(state, down_left, frame);
    render_timing_details(state, down_right, frame);
}

fn render_timing_details(state: &AppState, rect: Rect, frame: &mut Frame) {
    let tab_state = state.timing_table_state.borrow();

    if let Some(timings) = &state.sorted_timings
        && let Some(selected) = tab_state.selected()
        && let Some(timing) = timings.get(selected)
    {
        let block = Block::bordered().title(format!("{} details", timing.unit));

        let frontend = timing.frontend.map(fmt_dur).unwrap_or_default();
        let codegen = timing.codegen.map(fmt_dur).unwrap_or_default();
        let features = &timing.features;

        let front_str = format!("FRONTEND: {frontend}\n");
        let code_str = format!("CODEGEN: {codegen}\n");
        let features = format!("FEATURES: {}\n", features.join(", "));
        let p = Paragraph::new([front_str, code_str, features].as_slice())
            .wrap(ratatui::widgets::Wrap { trim: false })
            .block(block);
        frame.render_widget(p, rect);
    } else {
        let block = Block::bordered().title("Timing details");
        frame.render_widget(block, rect);
    }
}

fn render_timings(state: &AppState, rect: Rect, frame: &mut Frame) {
    let mut longest_unit = 4;
    for timing in &state.summary.timings {
        let unit = timing.unit.clone();
        if unit.len() > longest_unit {
            longest_unit = unit.len()
        }
    }

    let sort_string = format!(
        "sort <s|S>: {} ",
        match state.sort_order {
            config::Order::Descending => "desc",
            config::Order::Ascending => "asce",
        }
    );
    let search_string = format!("find <f>: {:_<10}", state.search_term.peek());
    let instructions = "scroll <up|down>, quit <q>";
    let block = Block::bordered()
        .title("TIMINGS")
        .title_top(Line::from(sort_string).bold().alignment(layout::HorizontalAlignment::Right))
        .title_bottom(search_string.to_line().bold().alignment(layout::HorizontalAlignment::Right))
        .title_bottom(instructions.to_line().bold().alignment(layout::HorizontalAlignment::Left));

    let inner = block.inner(rect);
    let x_start = inner.x + inner.width.saturating_sub(10.max(state.search_term.len() as u16));
    let x = inner.width.min(x_start + state.search_term.cursor as u16);
    let y = inner.y + inner.height;

    match state.focus {
        super::app_state::FocusElement::SearchInput => frame.set_cursor_position((x, y)),
        super::app_state::FocusElement::TimingTable => {}
    }

    let head_row = Row::new(["UNIT", "TOTAL"]);

    let mut rows = Vec::new();
    if let Some(timings) = &state.sorted_timings {
        for timing in timings {
            let unit = timing.unit.clone();
            let total = fmt_dur(timing.total).clone();
            let row = Row::new([unit, total]);
            rows.push(row);
        }
    }
    let widths = [Constraint::Max(longest_unit as u16), Constraint::Fill(1)];

    let table = Table::new(rows, widths)
        .header(head_row)
        .block(block)
        .highlight_symbol(">")
        .row_highlight_style(Style::new().bold());
    let mut t_state = state.timing_table_state.borrow_mut();
    frame.render_stateful_widget(table, rect, &mut *t_state);
}

fn render_summary(state: &AppState, rect: Rect, frame: &mut Frame) {
    let block = Block::bordered().title("SUMMARY");

    let targets = state.summary.targets.join(", ");
    let fresh = state.summary.fresh_units.to_string();
    let dirty = state.summary.dirty_units.to_string();
    let total = state.summary.total_units.to_string();
    let concurrency = format!("{} {}", state.summary.max_concurency, state.summary.concurency_details);
    let build_start = state.summary.build_start.to_string();
    let total_time = fmt_dur(state.summary.total_time);
    let rustc = state.summary.rustc.join(", ");

    let rows = [
        Row::new(["TARGETS: ", &targets]),
        Row::new(["PROFILE: ", &state.summary.profile]),
        Row::new(["FRESH UNITS: ", &fresh]),
        Row::new(["DIRTY UNITS: ", &dirty]),
        Row::new(["TOTAL UNITS: ", &total]),
        Row::new(["MAX CONCURRENCY: ", &concurrency]),
        Row::new(["BUILD START: ", &build_start]),
        Row::new(["TOTAL TIME: ", &total_time]),
        Row::new(["RUSTC: ", &rustc]),
    ];
    let widths = [Constraint::Max(20), Constraint::Fill(1)];

    let table = Table::new(rows, widths).block(block);

    frame.render_widget(table, rect);
}
