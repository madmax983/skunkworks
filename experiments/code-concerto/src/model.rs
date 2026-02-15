use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Instrument {
    Pad,        // Structs/Enums (Background chords)
    Lead,       // Functions (Melody)
    Bass,       // Impls (Foundation)
    Percussion, // Loops/Control Flow
    Arpeggio,   // Match arms / If branches
}

#[derive(Debug, Clone)]
pub enum MusicalEvent {
    NoteOn {
        #[allow(dead_code)]
        instrument: Instrument,
        #[allow(dead_code)]
        pitch: f32, // Frequency in Hz
        #[allow(dead_code)]
        volume: f32,
        duration: Duration,
    },
    Wait(Duration),
}

#[derive(Debug, Clone)]
pub struct Section {
    pub name: String,
    pub kind: SectionKind,
    pub depth: usize,
    pub events: Vec<MusicalEvent>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum SectionKind {
    Module,
    Struct,
    Enum,
    Impl,
    Function,
    Block,
}

#[derive(Debug, Default)]
pub struct CodeConcerto {
    pub sections: Vec<Section>,
    pub total_duration: Duration,
}

impl CodeConcerto {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_section(&mut self, section: Section) {
        // Calculate section duration
        let section_duration: Duration = section.events.iter().map(|e| match e {
            MusicalEvent::NoteOn { duration, .. } => *duration,
            MusicalEvent::Wait(d) => *d,
        }).sum();

        self.total_duration += section_duration;
        self.sections.push(section);
    }
}
