use std::cell::RefCell;

use line_buffer::LineBuffer;
use ratatui::widgets::TableState;

use crate::{
    config::Config,
    parser::{Summary, Timing},
    sorting::sort_timings,
    ui::input_handler::Input,
};

pub enum FocusElement {
    TimingTable,
    SearchInput,
}

pub struct AppState {
    pub summary: Summary,
    pub config: Config,
    pub search_term: LineBuffer,
    pub focus: FocusElement,
    pub should_quit: bool,
    pub timing_table_state: RefCell<TableState>,
    pub sorted_timings: Option<Vec<Timing>>,
}

impl AppState {
    pub fn init(summary: Summary, config: Config) -> Self {
        let mut search_term = LineBuffer::new();
        search_term.insert_str(&config.search);
        let mut tstate = TableState::new();
        tstate.select_first();
        Self {
            summary,
            config,
            search_term,
            focus: FocusElement::TimingTable,
            should_quit: false,
            timing_table_state: RefCell::new(tstate),
            sorted_timings: None,
        }
    }
    pub fn process_input(&mut self, input: Input) {
        match self.focus {
            FocusElement::TimingTable => match input {
                Input::Char('q') => self.should_quit = true,
                Input::Char('f') => self.focus = FocusElement::SearchInput,
                Input::Char('S') => self.config.order.toggle_next(),
                Input::Char('s') => self.config.order.toggle_prev(),
                Input::Up => self.timing_table_state.get_mut().select_previous(),
                Input::Down => self.timing_table_state.get_mut().select_next(),
                Input::Quit => self.should_quit = true,
                _ => {}
            },
            FocusElement::SearchInput => match input {
                Input::Esc | Input::Enter => self.focus = FocusElement::TimingTable,
                Input::Char(c) => self.search_term.insert_char(c),
                Input::Delete => self.search_term.delete(),
                Input::BackSpace => self.search_term.backspace(),
                Input::Left => self.search_term.move_left(),
                Input::Right => self.search_term.move_right(),
                Input::Quit => self.should_quit = true,
                _ => {}
            },
        }
    }
    pub fn sort_timings(&mut self) {
        self.config.search = self.search_term.as_str().to_string();
        let mut timings = self.summary.timings.clone();
        sort_timings(&mut timings, &self.config);
        self.sorted_timings = Some(timings);
    }
}
