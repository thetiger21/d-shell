use crossterm::event::MediaKeyCode as CrossMediaKeyCode;

use super::super::local::runtime::event_api::*;

impl From<CrossMediaKeyCode> for MediaKeyCode {
    fn from(value: CrossMediaKeyCode) -> Self {
        match value {
            CrossMediaKeyCode::FastForward => MediaKeyCode::FastForward,
            CrossMediaKeyCode::LowerVolume => MediaKeyCode::LowerVolume,
            CrossMediaKeyCode::MuteVolume => MediaKeyCode::MuteVolume,
            CrossMediaKeyCode::Pause => MediaKeyCode::Pause,
            CrossMediaKeyCode::Play => MediaKeyCode::Play,
            CrossMediaKeyCode::PlayPause => MediaKeyCode::PlayPause,
            CrossMediaKeyCode::RaiseVolume => MediaKeyCode::RaiseVolume,
            CrossMediaKeyCode::Record => MediaKeyCode::Record,
            CrossMediaKeyCode::Reverse => MediaKeyCode::Reverse,
            CrossMediaKeyCode::Rewind => MediaKeyCode::Rewind,
            CrossMediaKeyCode::Stop => MediaKeyCode::Stop,
            CrossMediaKeyCode::TrackNext => MediaKeyCode::TrackNext,
            CrossMediaKeyCode::TrackPrevious => MediaKeyCode::TrackPrevious,
        }
    }
}
