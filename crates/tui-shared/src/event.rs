use crossterm::event::{
    Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode,
    ModifierKeyCode, MouseButton, MouseEvent, MouseEventKind,
};
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostEvent {
    Key(GhostKeyEvent),
    Mouse(GhostMouseEvent),
    Resize(u16, u16),
    FocusGained,
    FocusLost,
    Paste(String),
}

impl From<Event> for GhostEvent {
    fn from(event: Event) -> Self {
        match event {
            Event::Key(e) => GhostEvent::Key(e.into()),
            Event::Mouse(e) => GhostEvent::Mouse(e.into()),
            Event::Resize(w, h) => GhostEvent::Resize(w, h),
            Event::FocusGained => GhostEvent::FocusGained,
            Event::FocusLost => GhostEvent::FocusLost,
            Event::Paste(s) => GhostEvent::Paste(s),
        }
    }
}

impl From<GhostEvent> for Event {
    fn from(event: GhostEvent) -> Self {
        match event {
            GhostEvent::Key(e) => Event::Key(e.into()),
            GhostEvent::Mouse(e) => Event::Mouse(e.into()),
            GhostEvent::Resize(w, h) => Event::Resize(w, h),
            GhostEvent::FocusGained => Event::FocusGained,
            GhostEvent::FocusLost => Event::FocusLost,
            GhostEvent::Paste(s) => Event::Paste(s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GhostKeyEvent {
    pub code: GhostKeyCode,
    pub modifiers: u8,
    pub kind: GhostKeyEventKind,
    pub state: u8,
}

impl From<KeyEvent> for GhostKeyEvent {
    fn from(event: KeyEvent) -> Self {
        Self {
            code: event.code.into(),
            modifiers: event.modifiers.bits(),
            kind: event.kind.into(),
            state: event.state.bits(),
        }
    }
}

impl From<GhostKeyEvent> for KeyEvent {
    fn from(event: GhostKeyEvent) -> Self {
        Self {
            code: event.code.into(),
            modifiers: KeyModifiers::from_bits_truncate(event.modifiers),
            kind: event.kind.into(),
            state: KeyEventState::from_bits_truncate(event.state),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostKeyCode {
    Backspace,
    Enter,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
    Tab,
    BackTab,
    Delete,
    Insert,
    F(u8),
    Char(char),
    Null,
    Esc,
    CapsLock,
    ScrollLock,
    NumLock,
    PrintScreen,
    Pause,
    Menu,
    KeypadBegin,
    Media(GhostMediaKeyCode),
    Modifier(GhostModifierKeyCode),
}

impl From<KeyCode> for GhostKeyCode {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Backspace => GhostKeyCode::Backspace,
            KeyCode::Enter => GhostKeyCode::Enter,
            KeyCode::Left => GhostKeyCode::Left,
            KeyCode::Right => GhostKeyCode::Right,
            KeyCode::Up => GhostKeyCode::Up,
            KeyCode::Down => GhostKeyCode::Down,
            KeyCode::Home => GhostKeyCode::Home,
            KeyCode::End => GhostKeyCode::End,
            KeyCode::PageUp => GhostKeyCode::PageUp,
            KeyCode::PageDown => GhostKeyCode::PageDown,
            KeyCode::Tab => GhostKeyCode::Tab,
            KeyCode::BackTab => GhostKeyCode::BackTab,
            KeyCode::Delete => GhostKeyCode::Delete,
            KeyCode::Insert => GhostKeyCode::Insert,
            KeyCode::F(n) => GhostKeyCode::F(n),
            KeyCode::Char(c) => GhostKeyCode::Char(c),
            KeyCode::Null => GhostKeyCode::Null,
            KeyCode::Esc => GhostKeyCode::Esc,
            KeyCode::CapsLock => GhostKeyCode::CapsLock,
            KeyCode::ScrollLock => GhostKeyCode::ScrollLock,
            KeyCode::NumLock => GhostKeyCode::NumLock,
            KeyCode::PrintScreen => GhostKeyCode::PrintScreen,
            KeyCode::Pause => GhostKeyCode::Pause,
            KeyCode::Menu => GhostKeyCode::Menu,
            KeyCode::KeypadBegin => GhostKeyCode::KeypadBegin,
            KeyCode::Media(m) => GhostKeyCode::Media(m.into()),
            KeyCode::Modifier(m) => GhostKeyCode::Modifier(m.into()),
        }
    }
}

impl From<GhostKeyCode> for KeyCode {
    fn from(code: GhostKeyCode) -> Self {
        match code {
            GhostKeyCode::Backspace => KeyCode::Backspace,
            GhostKeyCode::Enter => KeyCode::Enter,
            GhostKeyCode::Left => KeyCode::Left,
            GhostKeyCode::Right => KeyCode::Right,
            GhostKeyCode::Up => KeyCode::Up,
            GhostKeyCode::Down => KeyCode::Down,
            GhostKeyCode::Home => KeyCode::Home,
            GhostKeyCode::End => KeyCode::End,
            GhostKeyCode::PageUp => KeyCode::PageUp,
            GhostKeyCode::PageDown => KeyCode::PageDown,
            GhostKeyCode::Tab => KeyCode::Tab,
            GhostKeyCode::BackTab => KeyCode::BackTab,
            GhostKeyCode::Delete => KeyCode::Delete,
            GhostKeyCode::Insert => KeyCode::Insert,
            GhostKeyCode::F(n) => KeyCode::F(n),
            GhostKeyCode::Char(c) => KeyCode::Char(c),
            GhostKeyCode::Null => KeyCode::Null,
            GhostKeyCode::Esc => KeyCode::Esc,
            GhostKeyCode::CapsLock => KeyCode::CapsLock,
            GhostKeyCode::ScrollLock => KeyCode::ScrollLock,
            GhostKeyCode::NumLock => KeyCode::NumLock,
            GhostKeyCode::PrintScreen => KeyCode::PrintScreen,
            GhostKeyCode::Pause => KeyCode::Pause,
            GhostKeyCode::Menu => KeyCode::Menu,
            GhostKeyCode::KeypadBegin => KeyCode::KeypadBegin,
            GhostKeyCode::Media(m) => KeyCode::Media(m.into()),
            GhostKeyCode::Modifier(m) => KeyCode::Modifier(m.into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostMediaKeyCode {
    Play,
    Pause,
    PlayPause,
    Reverse,
    Stop,
    FastForward,
    Rewind,
    TrackNext,
    TrackPrevious,
    Record,
    LowerVolume,
    RaiseVolume,
    MuteVolume,
}

impl From<MediaKeyCode> for GhostMediaKeyCode {
    fn from(code: MediaKeyCode) -> Self {
        match code {
            MediaKeyCode::Play => GhostMediaKeyCode::Play,
            MediaKeyCode::Pause => GhostMediaKeyCode::Pause,
            MediaKeyCode::PlayPause => GhostMediaKeyCode::PlayPause,
            MediaKeyCode::Reverse => GhostMediaKeyCode::Reverse,
            MediaKeyCode::Stop => GhostMediaKeyCode::Stop,
            MediaKeyCode::FastForward => GhostMediaKeyCode::FastForward,
            MediaKeyCode::Rewind => GhostMediaKeyCode::Rewind,
            MediaKeyCode::TrackNext => GhostMediaKeyCode::TrackNext,
            MediaKeyCode::TrackPrevious => GhostMediaKeyCode::TrackPrevious,
            MediaKeyCode::Record => GhostMediaKeyCode::Record,
            MediaKeyCode::LowerVolume => GhostMediaKeyCode::LowerVolume,
            MediaKeyCode::RaiseVolume => GhostMediaKeyCode::RaiseVolume,
            MediaKeyCode::MuteVolume => GhostMediaKeyCode::MuteVolume,
        }
    }
}

impl From<GhostMediaKeyCode> for MediaKeyCode {
    fn from(code: GhostMediaKeyCode) -> Self {
        match code {
            GhostMediaKeyCode::Play => MediaKeyCode::Play,
            GhostMediaKeyCode::Pause => MediaKeyCode::Pause,
            GhostMediaKeyCode::PlayPause => MediaKeyCode::PlayPause,
            GhostMediaKeyCode::Reverse => MediaKeyCode::Reverse,
            GhostMediaKeyCode::Stop => MediaKeyCode::Stop,
            GhostMediaKeyCode::FastForward => MediaKeyCode::FastForward,
            GhostMediaKeyCode::Rewind => MediaKeyCode::Rewind,
            GhostMediaKeyCode::TrackNext => MediaKeyCode::TrackNext,
            GhostMediaKeyCode::TrackPrevious => MediaKeyCode::TrackPrevious,
            GhostMediaKeyCode::Record => MediaKeyCode::Record,
            GhostMediaKeyCode::LowerVolume => MediaKeyCode::LowerVolume,
            GhostMediaKeyCode::RaiseVolume => MediaKeyCode::RaiseVolume,
            GhostMediaKeyCode::MuteVolume => MediaKeyCode::MuteVolume,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostModifierKeyCode {
    LeftShift,
    LeftControl,
    LeftAlt,
    LeftSuper,
    LeftHyper,
    LeftMeta,
    RightShift,
    RightControl,
    RightAlt,
    RightSuper,
    RightHyper,
    RightMeta,
    IsoLevel3Shift,
    IsoLevel5Shift,
}

impl From<ModifierKeyCode> for GhostModifierKeyCode {
    fn from(code: ModifierKeyCode) -> Self {
        match code {
            ModifierKeyCode::LeftShift => GhostModifierKeyCode::LeftShift,
            ModifierKeyCode::LeftControl => GhostModifierKeyCode::LeftControl,
            ModifierKeyCode::LeftAlt => GhostModifierKeyCode::LeftAlt,
            ModifierKeyCode::LeftSuper => GhostModifierKeyCode::LeftSuper,
            ModifierKeyCode::LeftHyper => GhostModifierKeyCode::LeftHyper,
            ModifierKeyCode::LeftMeta => GhostModifierKeyCode::LeftMeta,
            ModifierKeyCode::RightShift => GhostModifierKeyCode::RightShift,
            ModifierKeyCode::RightControl => GhostModifierKeyCode::RightControl,
            ModifierKeyCode::RightAlt => GhostModifierKeyCode::RightAlt,
            ModifierKeyCode::RightSuper => GhostModifierKeyCode::RightSuper,
            ModifierKeyCode::RightHyper => GhostModifierKeyCode::RightHyper,
            ModifierKeyCode::RightMeta => GhostModifierKeyCode::RightMeta,
            ModifierKeyCode::IsoLevel3Shift => GhostModifierKeyCode::IsoLevel3Shift,
            ModifierKeyCode::IsoLevel5Shift => GhostModifierKeyCode::IsoLevel5Shift,
        }
    }
}

impl From<GhostModifierKeyCode> for ModifierKeyCode {
    fn from(code: GhostModifierKeyCode) -> Self {
        match code {
            GhostModifierKeyCode::LeftShift => ModifierKeyCode::LeftShift,
            GhostModifierKeyCode::LeftControl => ModifierKeyCode::LeftControl,
            GhostModifierKeyCode::LeftAlt => ModifierKeyCode::LeftAlt,
            GhostModifierKeyCode::LeftSuper => ModifierKeyCode::LeftSuper,
            GhostModifierKeyCode::LeftHyper => ModifierKeyCode::LeftHyper,
            GhostModifierKeyCode::LeftMeta => ModifierKeyCode::LeftMeta,
            GhostModifierKeyCode::RightShift => ModifierKeyCode::RightShift,
            GhostModifierKeyCode::RightControl => ModifierKeyCode::RightControl,
            GhostModifierKeyCode::RightAlt => ModifierKeyCode::RightAlt,
            GhostModifierKeyCode::RightSuper => ModifierKeyCode::RightSuper,
            GhostModifierKeyCode::RightHyper => ModifierKeyCode::RightHyper,
            GhostModifierKeyCode::RightMeta => ModifierKeyCode::RightMeta,
            GhostModifierKeyCode::IsoLevel3Shift => ModifierKeyCode::IsoLevel3Shift,
            GhostModifierKeyCode::IsoLevel5Shift => ModifierKeyCode::IsoLevel5Shift,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostKeyEventKind {
    Press,
    Repeat,
    Release,
}

impl From<KeyEventKind> for GhostKeyEventKind {
    fn from(kind: KeyEventKind) -> Self {
        match kind {
            KeyEventKind::Press => GhostKeyEventKind::Press,
            KeyEventKind::Repeat => GhostKeyEventKind::Repeat,
            KeyEventKind::Release => GhostKeyEventKind::Release,
        }
    }
}

impl From<GhostKeyEventKind> for KeyEventKind {
    fn from(kind: GhostKeyEventKind) -> Self {
        match kind {
            GhostKeyEventKind::Press => KeyEventKind::Press,
            GhostKeyEventKind::Repeat => KeyEventKind::Repeat,
            GhostKeyEventKind::Release => KeyEventKind::Release,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GhostMouseEvent {
    pub kind: GhostMouseEventKind,
    pub column: u16,
    pub row: u16,
    pub modifiers: u8,
}

impl From<MouseEvent> for GhostMouseEvent {
    fn from(event: MouseEvent) -> Self {
        Self {
            kind: event.kind.into(),
            column: event.column,
            row: event.row,
            modifiers: event.modifiers.bits(),
        }
    }
}

impl From<GhostMouseEvent> for MouseEvent {
    fn from(event: GhostMouseEvent) -> Self {
        Self {
            kind: event.kind.into(),
            column: event.column,
            row: event.row,
            modifiers: KeyModifiers::from_bits_truncate(event.modifiers),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostMouseEventKind {
    Down(GhostMouseButton),
    Up(GhostMouseButton),
    Drag(GhostMouseButton),
    Moved,
    ScrollDown,
    ScrollUp,
    ScrollLeft,
    ScrollRight,
}

impl From<MouseEventKind> for GhostMouseEventKind {
    fn from(kind: MouseEventKind) -> Self {
        match kind {
            MouseEventKind::Down(btn) => GhostMouseEventKind::Down(btn.into()),
            MouseEventKind::Up(btn) => GhostMouseEventKind::Up(btn.into()),
            MouseEventKind::Drag(btn) => GhostMouseEventKind::Drag(btn.into()),
            MouseEventKind::Moved => GhostMouseEventKind::Moved,
            MouseEventKind::ScrollDown => GhostMouseEventKind::ScrollDown,
            MouseEventKind::ScrollUp => GhostMouseEventKind::ScrollUp,
            MouseEventKind::ScrollLeft => GhostMouseEventKind::ScrollLeft,
            MouseEventKind::ScrollRight => GhostMouseEventKind::ScrollRight,
        }
    }
}

impl From<GhostMouseEventKind> for MouseEventKind {
    fn from(kind: GhostMouseEventKind) -> Self {
        match kind {
            GhostMouseEventKind::Down(btn) => MouseEventKind::Down(btn.into()),
            GhostMouseEventKind::Up(btn) => MouseEventKind::Up(btn.into()),
            GhostMouseEventKind::Drag(btn) => MouseEventKind::Drag(btn.into()),
            GhostMouseEventKind::Moved => MouseEventKind::Moved,
            GhostMouseEventKind::ScrollDown => MouseEventKind::ScrollDown,
            GhostMouseEventKind::ScrollUp => MouseEventKind::ScrollUp,
            GhostMouseEventKind::ScrollLeft => MouseEventKind::ScrollLeft,
            GhostMouseEventKind::ScrollRight => MouseEventKind::ScrollRight,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GhostMouseButton {
    Left,
    Right,
    Middle,
}

impl From<MouseButton> for GhostMouseButton {
    fn from(btn: MouseButton) -> Self {
        match btn {
            MouseButton::Left => GhostMouseButton::Left,
            MouseButton::Right => GhostMouseButton::Right,
            MouseButton::Middle => GhostMouseButton::Middle,
        }
    }
}

impl From<GhostMouseButton> for MouseButton {
    fn from(btn: GhostMouseButton) -> Self {
        match btn {
            GhostMouseButton::Left => MouseButton::Left,
            GhostMouseButton::Right => MouseButton::Right,
            GhostMouseButton::Middle => MouseButton::Middle,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedEvent {
    pub time: Duration,
    pub event: GhostEvent,
}

#[derive(Default)]
pub struct GhostRecorder {
    start_time: Option<Instant>,
    pub events: Vec<RecordedEvent>,
}

impl GhostRecorder {
    pub fn new() -> Self {
        Self {
            start_time: None,
            events: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.events.clear();
    }

    pub fn record(&mut self, event: Event) {
        if let Some(start) = self.start_time {
            let elapsed = start.elapsed();
            self.events.push(RecordedEvent {
                time: elapsed,
                event: event.into(),
            });
        }
    }

    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string(&self.events)
    }
}

pub struct GhostReplayer {
    start_time: Option<Instant>,
    events: Vec<RecordedEvent>,
    cursor: usize,
}

impl GhostReplayer {
    pub fn new(events: Vec<RecordedEvent>) -> Self {
        Self {
            start_time: None,
            events,
            cursor: 0,
        }
    }

    pub fn from_json(json: &str) -> serde_json::Result<Self> {
        let events: Vec<RecordedEvent> = serde_json::from_str(json)?;
        Ok(Self::new(events))
    }

    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
        self.cursor = 0;
    }

    pub fn poll(&mut self) -> Option<Event> {
        let start = self.start_time?;

        if self.cursor >= self.events.len() {
            return None;
        }

        let next_event = &self.events[self.cursor];
        if start.elapsed() >= next_event.time {
            self.cursor += 1;
            Some(next_event.event.clone().into())
        } else {
            None
        }
    }

    /// Peek at the time until the next event
    pub fn time_until_next(&self) -> Option<Duration> {
        let start = self.start_time?;
        if self.cursor >= self.events.len() {
            return None;
        }
        let next_event = &self.events[self.cursor];
        let elapsed = start.elapsed();
        if next_event.time > elapsed {
            Some(next_event.time - elapsed)
        } else {
            Some(Duration::ZERO)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_ghost_event_serialization() {
        // Create a crossterm event
        let original_event = Event::Key(KeyEvent {
            code: KeyCode::Char('a'),
            modifiers: KeyModifiers::CONTROL,
            kind: crossterm::event::KeyEventKind::Press,
            state: crossterm::event::KeyEventState::empty(),
        });

        // Convert to GhostEvent
        let ghost_event: GhostEvent = original_event.clone().into();

        // Serialize
        let json = serde_json::to_string(&ghost_event).expect("Failed to serialize");

        // Deserialize
        let deserialized_ghost: GhostEvent =
            serde_json::from_str(&json).expect("Failed to deserialize");

        // Convert back to Event
        let deserialized_event: Event = deserialized_ghost.into();

        // Assert equality
        assert_eq!(original_event, deserialized_event);
    }

    #[test]
    fn test_recorder_replayer() {
        let mut recorder = GhostRecorder::new();
        recorder.start();

        // Simulate recording
        let evt1 = Event::Key(KeyEvent::new(KeyCode::Char('a'), KeyModifiers::NONE));
        let evt2 = Event::Key(KeyEvent::new(KeyCode::Char('b'), KeyModifiers::NONE));

        // We can't easily simulate time passing in unit tests without mocking Instant,
        // so we'll just record them. In a real scenario, there would be a delay.
        recorder.record(evt1.clone());
        recorder.record(evt2.clone());

        let json = recorder.to_json().unwrap();

        let mut replayer = GhostReplayer::from_json(&json).unwrap();
        replayer.start();

        // Poll immediately - should get events if time diff is negligible
        // Note: This is flaky if we rely on exact timing.
        // But since we just recorded them, elapsed is very small.

        // To make this robust, we can just check the events are there.
        assert_eq!(replayer.events.len(), 2);
        assert_eq!(Event::from(replayer.events[0].event.clone()), evt1);
        assert_eq!(Event::from(replayer.events[1].event.clone()), evt2);
    }
}
