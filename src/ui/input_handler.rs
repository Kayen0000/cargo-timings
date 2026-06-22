use std::time::Duration;

use crossterm::event::{KeyCode, KeyModifiers};

pub struct InputHandler {
    poll_duration: Duration,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    Char(char),
    Enter,
    Left,
    Right,
    Up,
    Down,
    Esc,
    BackSpace,
    Delete,
    Quit,
}
impl InputHandler {
    pub fn new(poll_duration: Duration) -> Self {
        Self { poll_duration }
    }
    pub fn read_input(&self) -> Option<Input> {
        if !crossterm::event::poll(self.poll_duration).unwrap() {
            return None;
        }

        let event = crossterm::event::read().unwrap();

        let event = event.as_key_press_event()?;

        if event.modifiers.contains(KeyModifiers::CONTROL) && (event.code == KeyCode::Char('c') || event.code == KeyCode::Char('C')) {
            return Some(Input::Quit);
        }

        match event.code {
            KeyCode::Up => Some(Input::Up),
            KeyCode::Down => Some(Input::Down),
            KeyCode::Left => Some(Input::Left),
            KeyCode::Right => Some(Input::Right),
            KeyCode::Esc => Some(Input::Esc),
            KeyCode::Enter => Some(Input::Enter),
            KeyCode::Backspace => Some(Input::BackSpace),
            KeyCode::Delete => Some(Input::Delete),
            KeyCode::Char(c) => Some(Input::Char(c)),
            _ => None,
        }
    }
}
