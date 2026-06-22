use std::cell::RefCell;

use ratatui::widgets::TableState;

use crate::{
    config::{self, Config, Order},
    parser::{Summary, Timing},
    ui::{buffer_manager::SingleLineBufferManager, input_handler::Input},
};

pub enum FocusElement {
    TimingTable,
    SearchInput,
}

pub struct AppState {
    pub summary: Summary,
    pub sort_order: Order,
    pub search_term: SingleLineBufferManager,
    pub focus: FocusElement,
    pub should_quit: bool,
    pub timing_table_state: RefCell<TableState>,
    pub sorted_timings:Option<Vec<Timing>>,
}

impl AppState {
    pub fn init(summary: Summary, config: Config) -> Self {
        let search_term = SingleLineBufferManager::init_with(config.search.unwrap_or_default(), 0);
        let mut tstate = TableState::new();
        tstate.select_first();
        Self {
            summary,
            sort_order: config.order,
            search_term,
            focus: FocusElement::TimingTable,
            should_quit: false,
            timing_table_state: RefCell::new(tstate),
            sorted_timings:None,
        }
    }
    pub fn process_input(&mut self, input: Input) {
        match self.focus {
            FocusElement::TimingTable => match input {
                Input::Char('q') => self.should_quit = true,
                Input::Char('f') => self.focus = FocusElement::SearchInput,
                Input::Char('[') => {} // side_details.up
                Input::Char(']') => {} // side_details.down
                Input::Char('S') => self.sort_order.toggle_next(),
                Input::Char('s') => self.sort_order.toggle_prev(),
                Input::Up => self.timing_table_state.get_mut().select_previous(),
                Input::Down => self.timing_table_state.get_mut().select_next(),
                Input::Quit => self.should_quit = true,
                _ => {}
            },
            FocusElement::SearchInput => match input {
                Input::Esc => self.focus = FocusElement::TimingTable,
                Input::Char(c) => self.search_term.insert(c),
                Input::Delete => self.search_term.delete(),
                Input::BackSpace => self.search_term.backspace(),
                Input::Left => self.search_term.left(),
                Input::Right => self.search_term.right(),
                Input::Quit => self.should_quit = true,
                _ => {}
            },
        }
    }
    pub fn sort_timings(&mut self) {
        let mut timings = self.summary.timings.clone();
        match self.sort_order {
            config::Order::Ascending => timings.sort_by_key(|a| a.total),
            config::Order::Descending => timings.sort_by_key(|b| std::cmp::Reverse(b.total)),
        }

        timings.retain(|t| t.unit.contains(self.search_term.peek()));
        self.sorted_timings = Some(timings);
    }
}
