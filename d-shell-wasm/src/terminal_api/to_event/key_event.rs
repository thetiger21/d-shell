use crossterm::event::{
    KeyCode as CrossKeyCode, KeyEvent as CrossKeyEvent, KeyEventKind as CrossKeyEventKind,
    KeyEventState as CrossKeyEventState, KeyModifiers as CrossKeyModifiers,
    ModifierKeyCode as CrossModifierKeyCode,
};

use super::super::local::runtime::event_api::*;

impl From<CrossKeyEvent> for KeyEvent {
    fn from(value: CrossKeyEvent) -> Self {
        Self {
            code: value.code.into(),
            modifiers: value.modifiers.into(),
            kind: value.kind.into(),
            state: value.state.into(),
        }
    }
}

impl From<CrossKeyCode> for KeyCode {
    fn from(value: CrossKeyCode) -> Self {
        match value {
            CrossKeyCode::BackTab => KeyCode::BackTab,
            CrossKeyCode::Backspace => KeyCode::Backspace,
            CrossKeyCode::CapsLock => KeyCode::CapsLock,
            CrossKeyCode::Char(character) => KeyCode::Character(character),
            CrossKeyCode::Delete => KeyCode::Delete,
            CrossKeyCode::Down => KeyCode::Down,
            CrossKeyCode::End => KeyCode::End,
            CrossKeyCode::Enter => KeyCode::Enter,
            CrossKeyCode::Esc => KeyCode::Esc,
            CrossKeyCode::F(function) => KeyCode::F(function),
            CrossKeyCode::Home => KeyCode::Home,
            CrossKeyCode::Insert => KeyCode::Insert,
            CrossKeyCode::KeypadBegin => KeyCode::KeypadBegin,
            CrossKeyCode::Left => KeyCode::Left,
            CrossKeyCode::Media(media) => KeyCode::Media(media.into()),
            CrossKeyCode::Menu => KeyCode::Menu,
            CrossKeyCode::Modifier(modifier) => KeyCode::Modifier(modifier.into()),
            CrossKeyCode::Null => KeyCode::Null,
            CrossKeyCode::NumLock => KeyCode::NumLock,
            CrossKeyCode::PageDown => KeyCode::PageDown,
            CrossKeyCode::PageUp => KeyCode::PageUp,
            CrossKeyCode::Pause => KeyCode::Pause,
            CrossKeyCode::PrintScreen => KeyCode::PrintScreen,
            CrossKeyCode::Right => KeyCode::Right,
            CrossKeyCode::ScrollLock => KeyCode::ScrollLock,
            CrossKeyCode::Tab => KeyCode::Tab,
            CrossKeyCode::Up => KeyCode::Up,
        }
    }
}

impl From<CrossModifierKeyCode> for ModifierKeyCode {
    fn from(value: CrossModifierKeyCode) -> Self {
        match value {
            CrossModifierKeyCode::IsoLevel3Shift => ModifierKeyCode::IsoLevel3Shift,
            CrossModifierKeyCode::IsoLevel5Shift => ModifierKeyCode::IsoLevel5Shift,
            CrossModifierKeyCode::LeftAlt => ModifierKeyCode::LeftAlt,
            CrossModifierKeyCode::LeftControl => ModifierKeyCode::LeftControl,
            CrossModifierKeyCode::LeftHyper => ModifierKeyCode::LeftHyper,
            CrossModifierKeyCode::LeftMeta => ModifierKeyCode::LeftMeta,
            CrossModifierKeyCode::LeftShift => ModifierKeyCode::LeftShift,
            CrossModifierKeyCode::LeftSuper => ModifierKeyCode::LeftSuper,
            CrossModifierKeyCode::RightAlt => ModifierKeyCode::RightAlt,
            CrossModifierKeyCode::RightControl => ModifierKeyCode::RightControl,
            CrossModifierKeyCode::RightHyper => ModifierKeyCode::RightHyper,
            CrossModifierKeyCode::RightMeta => ModifierKeyCode::RightMeta,
            CrossModifierKeyCode::RightShift => ModifierKeyCode::RightShift,
            CrossModifierKeyCode::RightSuper => ModifierKeyCode::RightSuper,
        }
    }
}

impl From<CrossKeyModifiers> for KeyModifiers {
    fn from(value: CrossKeyModifiers) -> Self {
        let mut bits = KeyModifiers::empty();

        if value.contains(CrossKeyModifiers::SHIFT) {
            bits |= KeyModifiers::SHIFT;
        }
        if value.contains(CrossKeyModifiers::CONTROL) {
            bits |= KeyModifiers::CONTROL;
        }
        if value.contains(CrossKeyModifiers::ALT) {
            bits |= KeyModifiers::ALT;
        }
        if value.contains(CrossKeyModifiers::SUPER) {
            bits |= KeyModifiers::SUPER;
        }
        if value.contains(CrossKeyModifiers::HYPER) {
            bits |= KeyModifiers::HYPER;
        }
        if value.contains(CrossKeyModifiers::META) {
            bits |= KeyModifiers::META;
        }

        bits
    }
}

impl From<CrossKeyEventKind> for KeyEventKind {
    fn from(value: CrossKeyEventKind) -> Self {
        match value {
            CrossKeyEventKind::Press => KeyEventKind::Press,
            CrossKeyEventKind::Release => KeyEventKind::Release,
            CrossKeyEventKind::Repeat => KeyEventKind::Repeat,
        }
    }
}

impl From<CrossKeyEventState> for KeyEventState {
    fn from(value: CrossKeyEventState) -> Self {
        let mut bits = KeyEventState::empty();

        if value.contains(CrossKeyEventState::CAPS_LOCK) {
            bits |= KeyEventState::CAPS_LOCK;
        }
        if value.contains(CrossKeyEventState::KEYPAD) {
            bits |= KeyEventState::KEYPAD;
        }
        if value.contains(CrossKeyEventState::NONE) {
            bits |= KeyEventState::empty();
        }
        if value.contains(CrossKeyEventState::NUM_LOCK) {
            bits |= KeyEventState::NUM_LOCK;
        }

        bits
    }
}
