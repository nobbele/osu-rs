#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BeatmapFileSection {
    General,
    Editor,
    Metadata,
    Difficulty,
    Events,
    TimingPoints,
    Colours,
    HitObjects,
}

impl BeatmapFileSection {
    pub fn from_str(s: &str) -> Option<Self> {
        let v = match s {
            "General" => Self::General,
            "Editor" => Self::Editor,
            "Metadata" => Self::Metadata,
            "Difficulty" => Self::Difficulty,
            "Events" => Self::Events,
            "TimingPoints" => Self::TimingPoints,
            "Colours" => Self::Colours,
            "HitObjects" => Self::HitObjects,
            _ => return None,
        };
        Some(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Countdown {
    Normal,
    Half,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SampleSet {
    Normal,
    Soft,
    Drum,
}

impl SampleSet {
    pub fn from_str(s: &str) -> Option<Self> {
        let v = match s {
            "Normal" => Self::Normal,
            "Soft" => Self::Soft,
            "Drum" => Self::Drum,
            _ => return None,
        };
        Some(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}
