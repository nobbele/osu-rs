pub mod osz2;

#[cfg(feature = "serde")]
use serde_crate::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum BeatmapFileSection {
    General,
    Editor,
    Metadata,
    Difficulty,
    Events,
    TimingPoints,
    Colors,
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
            "Colours" => Self::Colors,
            "HitObjects" => Self::HitObjects,
            _ => return None,
        };
        Some(v)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum Countdown {
    Normal,
    Half,
    Double,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
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
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum Mode {
    Osu,
    Taiko,
    Catch,
    Mania,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct RGB<T> {
    pub r: T,
    pub g: T,
    pub b: T,
}

pub type ComboColor = RGB<u8>;

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum Event {
    Background {
        filename: String,
        offset: (u16, u16),
    },
    Video {
        start_time: u32,
        filename: String,
        offset: (u16, u16),
    },
    Break(std::ops::Range<u32>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct TimingPoint {
    pub time: i32,
    pub beat_length: f32,
    pub meter: u8,
    pub sample_set: Option<SampleSet>,
    pub sample_index: u8,
    pub volume: u8,
    pub uninherited: bool,
    pub effects: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum CurveType {
    Linear,
    Perfect,
    Bezier,
    Catmull,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub enum SpecificHitObject {
    Circle,
    Slider {
        curve_type: CurveType,
        curve_points: Vec<OsuPoint>,
        slides: u8,
        length: f32,
        edge_sounds: Vec<u8>,
    },
    Spinner {
        end_time: u32,
    },
    ManiaHold {
        // TODO
    },
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct HitSample {
    pub normal_set: u8,
    pub addition_set: u8,
    pub index: u8,
    pub volume: u8,
    pub filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct HitObject {
    pub position: (u16, u16),
    pub time: u32,
    pub specific: SpecificHitObject,
    pub hit_sound: u8,
    pub hit_sample: HitSample,
    pub new_combo: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct BeatmapGeneralData {
    pub audio_file_name: String,
    pub audio_lead_in: u32,
    pub preview_time: u32,
    pub countdown: Option<Countdown>,
    pub sample_set: SampleSet,
    pub stack_leniency: f32,
    pub mode: Mode,
    pub letterbox_in_breaks: bool,
    pub widescreen_storyboard: bool,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct BeatmapMetadata {
    pub title: String,
    pub title_unicode: String,
    pub artist: String,
    pub artist_unicode: String,
    pub creator: String,
    pub version: String,
    pub source: String,
    pub tags: String,
    pub beatmap_id: i32,
    pub beatmap_set_id: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct BeatmapDifficulty {
    pub hp: f32,
    pub cs: f32,
    pub od: f32,
    pub ar: f32,
    pub slider_multiplier: f32,
    pub slider_tick_rate: f32,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(Serialize, Deserialize),
    serde(crate = "serde_crate")
)]
pub struct BeatmapInfo {
    pub general_data: BeatmapGeneralData,
    pub metadata: BeatmapMetadata,
    pub difficulty: BeatmapDifficulty,
}

pub type OsuPoint = mint::Point2<i16>;

pub fn osu_point(x: i16, y: i16) -> OsuPoint {
    OsuPoint { x, y }
}
