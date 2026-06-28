use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug)]
pub enum InputEvent {
    Tab,
    BackTab,
    Quit,
    SearchFocus,
    Enter,
    Up,
    Down,
    Char(char),
    Backspace,
    Escape,
    Resize,
}

pub fn next_event(timeout: std::time::Duration) -> anyhow::Result<Option<InputEvent>> {
    if !event::poll(timeout)? {
        return Ok(None);
    }
    match event::read()? {
        Event::Key(k) => Ok(map_key(k)),
        Event::Resize(..) => Ok(Some(InputEvent::Resize)),
        _ => Ok(None),
    }
}

fn map_key(k: KeyEvent) -> Option<InputEvent> {
    match k.code {
        KeyCode::Tab => Some(InputEvent::Tab),
        KeyCode::BackTab => Some(InputEvent::BackTab),
        KeyCode::Char('q') if k.modifiers == KeyModifiers::NONE => Some(InputEvent::Quit),
        KeyCode::Char('/') if k.modifiers == KeyModifiers::NONE => Some(InputEvent::SearchFocus),
        KeyCode::Enter => Some(InputEvent::Enter),
        KeyCode::Up | KeyCode::Char('k') => Some(InputEvent::Up),
        KeyCode::Down | KeyCode::Char('j') => Some(InputEvent::Down),
        KeyCode::Backspace => Some(InputEvent::Backspace),
        KeyCode::Esc => Some(InputEvent::Escape),
        KeyCode::Char(c) => Some(InputEvent::Char(c)),
        _ => None,
    }
}
