use osu_types::{Countdown, Mode, SampleSet};

#[derive(Default)]
pub struct InParseGeneral<'a> {
    pub audio_file_name: Option<&'a str>,
    pub audio_lead_in: Option<u32>,
    pub preview_time: Option<u32>,
    pub countdown: Option<Countdown>,
    pub sample_set: Option<SampleSet>,
    pub stack_leniency: Option<f32>,
    pub mode: Option<Mode>,
    pub letterbox_in_breaks: Option<bool>,
    pub widescreen_storyboard: Option<bool>,
}

#[derive(Default)]
#[cfg(feature = "read-editor")]
pub struct InParseEditor {
    pub bookmarks: Vec<u32>,
    pub distance_spacing: Option<f32>,
    pub beat_divisor: Option<u8>,
    pub grid_size: Option<u8>,
    pub timeline_zoom: Option<f32>,
}

#[derive(Default)]
pub struct InParseMetadata<'a> {
    pub title: Option<&'a str>,
    pub title_unicode: Option<&'a str>,
    pub artist: Option<&'a str>,
    pub artist_unicode: Option<&'a str>,
    pub creator: Option<&'a str>,
    pub version: Option<&'a str>,
    pub source: Option<&'a str>,
    pub tags: Option<&'a str>,
    pub beatmap_id: Option<i32>,
    pub beatmap_set_id: Option<i32>,
}

#[derive(Default)]
pub struct InParseDifficulty {
    pub hp: Option<f32>,
    pub cs: Option<f32>,
    pub od: Option<f32>,
    pub ar: Option<f32>,
    pub slider_multiplier: Option<f32>,
    pub slider_tick_rate: Option<f32>,
}
