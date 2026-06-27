use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Row, Table},
};
use ratatui::{
    style::{Style, Stylize},
    text::Line,
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
    let mut max_len = 15;
    for t in &state.summary.timings {
        let line_len = t.unit.len() + fmt_dur(t.total).len() + 5;
        if max_len < line_len {
            max_len = line_len;
        }
    }

    let [left_side, right_side] = Layout::horizontal([Constraint::Max(max_len as u16), Constraint::Fill(1)]).areas(frame.area());

    let [timings_area, bot_area1] = Layout::vertical([Constraint::Fill(1), Constraint::Length(1)]).areas(left_side);

    let [top_area, details_area, bot_area2] = Layout::vertical([Constraint::Length(11), Constraint::Fill(1), Constraint::Length(1)]).areas(right_side);
    
    let bot_area = bot_area1.union(bot_area2);
    
    match state.focus {
        super::app_state::FocusElement::SearchInput => {
            // 41 is a length of string in instructions
            let x= bot_area.x + 41 + state.search_term.cursor_position() as u16;
            let y = bot_area.y;
            frame.set_cursor_position((x,y))
        },
        super::app_state::FocusElement::TimingTable => {}
    }

    render_timings(state, timings_area, frame);
    render_instructions(state, bot_area, frame);
    render_summary(state, top_area, frame);
    render_timing_details(state, details_area, frame);
}
fn render_instructions(state: &AppState, rect: Rect, frame: &mut Frame) {
    let status_line = match state.focus {
        super::app_state::FocusElement::TimingTable => vec![
            "Q: Quit".to_string(),
            "▲/▼: Scroll".to_string(),
            format!("S: Sort ({})", state.config.order.to_str()),
            format!("F: Find ({})", state.search_term.as_str()),
        ],
        super::app_state::FocusElement::SearchInput => vec![format!("Type to filter... <ESC/Enter> to return: {}", state.search_term.as_str())],
    };

    let p = Paragraph::new(Line::from(status_line.join("  |  ")).dark_gray());
    frame.render_widget(p, rect);
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
        let block = Block::bordered().title("Dependency details");
        let placeholder = Paragraph::new("Use <UP/DOWN> arrows to inspect a dependency").dark_gray().centered();
        frame.render_widget(placeholder.block(block), rect);
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

    let block = Block::bordered().title("TIMINGS");

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
        Row::new([Line::from("TARGETS:").dark_gray(), Line::from(targets).bold()]),
        Row::new([Line::from("PROFILE:").dark_gray(), Line::from(state.summary.profile.as_str()).bold()]),
        Row::new([Line::from("FRESH UNITS:").dark_gray(), Line::from(fresh).bold()]),
        Row::new([Line::from("DIRTY UNITS:").dark_gray(), Line::from(dirty).bold()]),
        Row::new([Line::from("TOTAL UNITS:").dark_gray(), Line::from(total).bold()]),
        Row::new([Line::from("MAX CONCURRENCY:").dark_gray(), Line::from(concurrency).bold()]),
        Row::new([Line::from("BUILD START:").dark_gray(), Line::from(build_start).bold()]),
        Row::new([Line::from("TOTAL TIME:").dark_gray(), Line::from(total_time).bold()]),
        Row::new([Line::from("RUSTC:").dark_gray(), Line::from(rustc).bold()]),
    ];
    let widths = [Constraint::Max(20), Constraint::Fill(1)];

    let table = Table::new(rows, widths).block(block);

    frame.render_widget(table, rect);
}
