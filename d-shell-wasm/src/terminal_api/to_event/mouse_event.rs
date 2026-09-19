use crossterm::event::{
    MouseButton as CrossMouseButton, MouseEvent as CrossMouseEvent,
    MouseEventKind as CrossMouseEventKind,
};

use super::super::local::runtime::event_api::*;

impl From<CrossMouseEvent> for MouseEvent {
    fn from(value: CrossMouseEvent) -> Self {
        Self {
            kind: value.kind.into(),
            column: value.column,
            row: value.row,
            modifiers: value.modifiers.into(),
        }
    }
}

impl From<CrossMouseEventKind> for MouseEventKind {
    fn from(value: CrossMouseEventKind) -> Self {
        match value {
            CrossMouseEventKind::Down(value) => MouseEventKind::Down(value.into()),
            CrossMouseEventKind::Up(value) => MouseEventKind::Up(value.into()),
            CrossMouseEventKind::Drag(value) => MouseEventKind::Drag(value.into()),
            CrossMouseEventKind::Moved => MouseEventKind::Moved,
            CrossMouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            CrossMouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
            CrossMouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
            CrossMouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
        }
    }
}

impl From<CrossMouseButton> for MouseButton {
    fn from(value: CrossMouseButton) -> Self {
        match value {
            CrossMouseButton::Left => MouseButton::Left,
            CrossMouseButton::Middle => MouseButton::Middle,
            CrossMouseButton::Right => MouseButton::Right,
        }
    }
}
