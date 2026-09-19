pub mod key_event;
pub mod media_key;
pub mod mouse_event;

use super::local::runtime::event_api::*;
use crossterm::event::Event as CrossEvent;

impl From<CrossEvent> for Event {
    fn from(value: CrossEvent) -> Self {
        match value {
            CrossEvent::FocusGained => Event::FocusGained,
            CrossEvent::FocusLost => Event::FocusLost,
            CrossEvent::Key(key) => Event::Key(key.into()),
            CrossEvent::Mouse(mouse) => Event::Mouse(mouse.into()),
            CrossEvent::Paste(pasted) => Event::Paste(pasted),
            CrossEvent::Resize(width, height) => Event::Resize((width, height)),
        }
    }
}
