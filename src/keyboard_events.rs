use crossterm::event::{Event, KeyCode, KeyModifiers};

pub enum KeyboardEvent {
    Quit,
    Close,
    Update,
    ScrollDown,
    ScrollUp,
    ScrollPageDown,
    ScrollPageUp,
    ScrollEnd,
    ScrollStart,
    Other,
}

pub fn get_keyboard_event(event: &Event, prev_event: &Option<Event>) -> KeyboardEvent {
    if let Event::Key(key) = event {
        return match (key.code, key.modifiers) {
            // Quit and close events
            (KeyCode::Char('q'), KeyModifiers::NONE)
            | (KeyCode::Char('c'), KeyModifiers::CONTROL) => KeyboardEvent::Quit,
            (KeyCode::Esc, KeyModifiers::NONE) => KeyboardEvent::Close,
            // Scroll events
            (KeyCode::Char('j'), KeyModifiers::NONE) => KeyboardEvent::ScrollDown,
            (KeyCode::Char('k'), KeyModifiers::NONE) => KeyboardEvent::ScrollUp,
            (KeyCode::Char('d'), KeyModifiers::CONTROL) => KeyboardEvent::ScrollPageDown,
            (KeyCode::Char('u'), KeyModifiers::CONTROL) => KeyboardEvent::ScrollPageUp,
            (KeyCode::Char('G'), KeyModifiers::SHIFT) => KeyboardEvent::ScrollEnd,
            (KeyCode::Char('g'), KeyModifiers::NONE) => {
                if let Some(prev_event) = prev_event {
                    if let Event::Key(prev_key) = prev_event {
                        return match (prev_key.code, prev_key.modifiers) {
                            (KeyCode::Char('g'), KeyModifiers::NONE) => KeyboardEvent::ScrollStart,
                            _ => KeyboardEvent::Other,
                        };
                    };
                }

                KeyboardEvent::Other
            }
            // Screen events
            (KeyCode::Char('U'), KeyModifiers::SHIFT) => KeyboardEvent::Update,
            // Drop the other
            _ => KeyboardEvent::Other,
        };
    }

    KeyboardEvent::Other
}
