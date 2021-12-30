#![allow(unused)]

use std::num::{ParseFloatError, ParseIntError};

use error::*;
use in_parse_types::*;
use osu_types::{
    BeatmapDifficulty, BeatmapFileSection, BeatmapGeneralData, BeatmapInfo, BeatmapMetadata,
    ComboColor, Countdown, Event, HitObject, HitSample, Mode, SampleSet, SpecificHitObject,
    TimingPoint,
};

pub mod error;
mod in_parse_types;

// TODO make these optional at runtime
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Beatmap {
    pub info: BeatmapInfo,
    pub events: Vec<Event>,
    pub colors: Vec<ComboColor>,
    pub timing_points: Vec<TimingPoint>,
    pub hit_objects: Vec<HitObject>,
}

pub struct BeatmapParseOptions {
    pub read_editor: bool,
    pub read_events: bool,
    pub read_colors: bool,
    pub read_timing_points: bool,
    pub read_hit_objects: bool,
}

impl Default for BeatmapParseOptions {
    fn default() -> Self {
        Self {
            read_editor: true,
            read_events: true,
            read_colors: false,
            read_timing_points: true,
            read_hit_objects: true,
        }
    }
}

pub fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((key.trim(), value.trim()))
}

pub fn load_file(
    path: impl AsRef<std::path::Path>,
    options: BeatmapParseOptions,
) -> OsuParserResult<Beatmap> {
    let content = std::fs::read_to_string(path).unwrap();
    load_content(&content, options)
}

#[derive(Default)]
struct ParseData<'a> {
    general: InParseGeneral<'a>,
    editor: InParseEditor,
    metadata: InParseMetadata<'a>,
    difficulty: InParseDifficulty,
    events: Vec<Event>,
    timing_points: Vec<TimingPoint>,
    colors: Vec<ComboColor>,
    hit_objects: Vec<HitObject>,
}

pub fn load_content(content: &str, options: BeatmapParseOptions) -> OsuParserResult<Beatmap> {
    let mut lines = content
        .lines()
        .filter(|&line| !line.trim().is_empty() && !line.starts_with("//"));

    let version_string: &str = lines
        .next()
        .ok_or((None, OsuParserError::BadFormat))?
        .trim_start_matches("osu file format v");
    let version: u8 = version_string
        .parse()
        .map_err(|e: ParseIntError| (None, e.into()))?;

    if version < 3 {
        return Err((None, OsuParserError::VersionTooOld(version)));
    } else if version > 14 {
        return Err((None, OsuParserError::VersionTooNew(version)));
    }

    let mut current_section: Option<BeatmapFileSection> = None;
    let mut data = ParseData::default();

    for line in lines {
        if line.starts_with("[") && line.ends_with("]") {
            let section_string = &line[1..line.len() - 1];
            let section = BeatmapFileSection::from_str(section_string).ok_or_else(|| {
                (
                    None,
                    OsuParserError::InvalidSection(section_string.to_owned()),
                )
            })?;
            current_section = Some(section);
            continue;
        }

        let current_section = current_section.ok_or((None, OsuParserError::DataOutsideSection))?;
        match current_section {
            BeatmapFileSection::General => {
                read_general_line(&mut data.general, line)
                    .map_err(|e| (Some(BeatmapFileSection::General), e))?;
            }
            BeatmapFileSection::Editor => {
                if options.read_editor {
                    read_editor_line(&mut data.editor, line)
                        .map_err(|e| (Some(BeatmapFileSection::Editor), e))?;
                };
            }
            BeatmapFileSection::Metadata => {
                read_metadata_line(&mut data.metadata, line)
                    .map_err(|e| (Some(BeatmapFileSection::Metadata), e))?;
            }
            BeatmapFileSection::Difficulty => {
                read_difficulty_line(&mut data.difficulty, line)
                    .map_err(|e| (Some(BeatmapFileSection::Difficulty), e))?;
            }
            BeatmapFileSection::Colors => {
                if options.read_colors {
                    //let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
                    todo!()
                };
            }
            BeatmapFileSection::Events => {
                if options.read_events {
                    let event =
                        read_event_line(line).map_err(|e| (Some(BeatmapFileSection::Events), e))?;
                    data.events.push(event);
                };
            }
            BeatmapFileSection::TimingPoints => {
                if options.read_timing_points {
                    let timing_point = read_timing_point_line(line)
                        .map_err(|e| (Some(BeatmapFileSection::TimingPoints), e))?;
                    data.timing_points.push(timing_point);
                };
            }
            BeatmapFileSection::HitObjects => {
                if options.read_hit_objects {
                    let hit_object = read_hitobject_line(line)
                        .map_err(|e| (Some(BeatmapFileSection::HitObjects), e))?;
                    data.hit_objects.push(hit_object);
                };
            }
        }
    }

    Ok(finalize_parse(data).map_err(|e| (None, e))?)
}

fn finalize_parse(data: ParseData) -> Result<Beatmap, OsuParserError> {
    let title = data.metadata.title.ok_or(OsuParserError::BadFormat)?;
    let artist = data.metadata.artist.ok_or(OsuParserError::BadFormat)?;
    let od = data.difficulty.od.ok_or(OsuParserError::BadFormat)?;
    Ok(Beatmap {
        info: BeatmapInfo {
            general_data: BeatmapGeneralData {
                audio_file_name: data
                    .general
                    .audio_file_name
                    .ok_or(OsuParserError::BadFormat)?
                    .to_owned(),
                audio_lead_in: data.general.audio_lead_in.unwrap_or(0),
                preview_time: data.general.preview_time.unwrap_or(0),
                countdown: data.general.countdown,
                sample_set: data.general.sample_set.unwrap_or(SampleSet::Normal),
                stack_leniency: data.general.stack_leniency.unwrap_or(0.5),
                mode: data.general.mode.unwrap_or(Mode::Osu),
                letterbox_in_breaks: data.general.letterbox_in_breaks.unwrap_or(false),
                widescreen_storyboard: data.general.widescreen_storyboard.unwrap_or(false),
            },
            metadata: BeatmapMetadata {
                title: title.to_owned(),
                title_unicode: data.metadata.title_unicode.unwrap_or(title).to_owned(),
                artist: artist.to_owned(),
                artist_unicode: data.metadata.artist_unicode.unwrap_or(artist).to_owned(),
                creator: data
                    .metadata
                    .creator
                    .ok_or(OsuParserError::BadFormat)?
                    .to_owned(),
                version: data
                    .metadata
                    .version
                    .ok_or(OsuParserError::BadFormat)?
                    .to_owned(),
                source: data.metadata.source.unwrap_or("").to_owned(),
                tags: data.metadata.tags.unwrap_or("").to_owned(),
                beatmap_id: data.metadata.beatmap_id.unwrap_or(-1).to_owned(),
                beatmap_set_id: data.metadata.beatmap_set_id.unwrap_or(-1).to_owned(),
            },
            difficulty: BeatmapDifficulty {
                hp: data.difficulty.hp.ok_or(OsuParserError::BadFormat)?,
                cs: data.difficulty.cs.ok_or(OsuParserError::BadFormat)?,
                od,
                ar: data.difficulty.ar.unwrap_or(od),
                slider_multiplier: data
                    .difficulty
                    .slider_multiplier
                    .ok_or(OsuParserError::BadFormat)?,
                slider_tick_rate: data
                    .difficulty
                    .slider_tick_rate
                    .ok_or(OsuParserError::BadFormat)?,
            },
        },
        events: data.events,
        colors: data.colors,
        timing_points: data.timing_points,
        hit_objects: data.hit_objects,
    })
}

fn read_general_line<'a>(
    general: &mut InParseGeneral<'a>,
    line: &'a str,
) -> Result<(), OsuParserError> {
    let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
    match key {
        "AudioFilename" => general.audio_file_name = Some(value),
        "AudioLeadIn" => general.audio_lead_in = Some(value.parse()?),
        "PreviewTime" => general.preview_time = Some(value.parse()?),
        "Countdown" => {
            general.countdown = match value.parse::<u8>()? {
                0 => None,
                1 => Some(Countdown::Normal),
                2 => Some(Countdown::Half),
                3 => Some(Countdown::Double),
                _ => return Err(OsuParserError::BadFormat),
            }
        }
        "SampleSet" => {
            general.sample_set = Some(SampleSet::from_str(value).ok_or(OsuParserError::BadFormat)?);
        }
        "StackLeniency" => general.stack_leniency = Some(value.parse()?),
        "Mode" => {
            general.mode = Some(match value.parse::<u8>()? {
                0 => Mode::Osu,
                1 => Mode::Taiko,
                2 => Mode::Catch,
                3 => Mode::Mania,
                _ => return Err(OsuParserError::BadFormat),
            })
        }
        "LetterboxInBreaks" => general.letterbox_in_breaks = Some(value.parse::<u8>()? != 0),
        "WidescreenStoryboard" => general.widescreen_storyboard = Some(value.parse::<u8>()? != 0),
        "AudioHash" => { /* Ignore */ }
        _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
    }

    Ok(())
}

fn read_editor_line(editor: &mut InParseEditor, line: &str) -> Result<(), OsuParserError> {
    let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
    match key {
        "Bookmarks" => {
            editor.bookmarks = value
                .split(',')
                .map(|s| s.parse())
                .collect::<Result<_, _>>()?
        }
        "DistanceSpacing" => editor.distance_spacing = Some(value.parse()?),
        "BeatDivisor" => editor.beat_divisor = Some(value.parse()?),
        "GridSize" => editor.grid_size = Some(value.parse()?),
        "TimelineZoom" => editor.timeline_zoom = Some(value.parse()?),
        _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
    }
    Ok(())
}

fn read_metadata_line<'a>(
    metadata: &mut InParseMetadata<'a>,
    line: &'a str,
) -> Result<(), OsuParserError> {
    let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
    match key {
        "Title" => metadata.title = Some(value),
        "TitleUnicode" => metadata.title_unicode = Some(value),
        "Artist" => metadata.artist = Some(value),
        "ArtistUnicode" => metadata.artist_unicode = Some(value),
        "Creator" => metadata.creator = Some(value),
        "Version" => metadata.version = Some(value),
        "Source" => metadata.source = Some(value),
        "Tags" => metadata.tags = Some(value),
        "BeatmapID" => metadata.beatmap_id = Some(value.parse()?),
        "BeatmapSetID" => metadata.beatmap_set_id = Some(value.parse()?),
        _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
    }
    Ok(())
}

fn read_difficulty_line(
    difficulty: &mut InParseDifficulty,
    line: &str,
) -> Result<(), OsuParserError> {
    let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
    match key {
        "HPDrainRate" => difficulty.hp = Some(value.parse()?),
        "CircleSize" => difficulty.cs = Some(value.parse()?),
        "OverallDifficulty" => difficulty.od = Some(value.parse()?),
        "ApproachRate" => difficulty.ar = Some(value.parse()?),
        "SliderMultiplier" => difficulty.slider_multiplier = Some(value.parse()?),
        "SliderTickRate" => difficulty.slider_tick_rate = Some(value.parse()?),
        _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
    }
    Ok(())
}

fn read_event_line(line: &str) -> Result<Event, OsuParserError> {
    let mut entries = line.split(',');
    let event_type = entries.next().ok_or(OsuParserError::BadFormat)?;
    let event = match event_type {
        "0" => {
            // this is always 0 for some reason
            entries.next();
            let filename = entries.next().ok_or(OsuParserError::BadFormat)?;
            let x_offset = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
            let y_offset = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
            Event::Background {
                filename: filename.to_owned(),
                offset: (x_offset, y_offset),
            }
        }
        "1" | "Video" => {
            let start_time = entries.next().ok_or(OsuParserError::BadFormat)?;
            let filename = entries.next().ok_or(OsuParserError::BadFormat)?;
            let x_offset = entries.next().ok_or(OsuParserError::BadFormat)?;
            let y_offset = entries.next().ok_or(OsuParserError::BadFormat)?;
            Event::Video {
                start_time: start_time.parse()?,
                filename: filename.to_owned(),
                offset: (x_offset.parse()?, y_offset.parse()?),
            }
        }
        "2" | "Break" => {
            let start_time: u32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
            let end_time: u32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
            Event::Break(start_time..end_time)
        }
        "3" => {
            println!("Ignoring storyboard...");
            Event::Break(0..0)
        }
        _ => return Err(OsuParserError::BadFormat),
    };
    Ok(event)
}

fn read_timing_point_line(line: &str) -> Result<TimingPoint, OsuParserError> {
    let mut entries = line.split(',');
    let time: f32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let beat_length: f32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let meter: u8 = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
    let sample_set: u32 = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
    let sample_index: u8 = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
    let volume: u8 = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
    let uninherited: bool = entries
        .next()
        .map(|v| v.parse::<u32>())
        .transpose()?
        .unwrap_or(0)
        != 0;
    let effects: u8 = entries.next().map(|v| v.parse()).transpose()?.unwrap_or(0);
    Ok(TimingPoint {
        time: time as i32,
        beat_length,
        meter,
        sample_set: match sample_set {
            0 => None,
            1 => Some(SampleSet::Normal),
            2 => Some(SampleSet::Soft),
            3 => Some(SampleSet::Drum),
            _ => return Err(OsuParserError::BadFormat),
        },
        sample_index,
        volume,
        uninherited,
        effects,
    })
}

fn parse_hit_sample(part: &str) -> Result<HitSample, OsuParserError> {
    let mut split = part.split(':');
    let normal_set = split.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let addition_set = split.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let index = split.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let volume = split.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let filename = split.next().unwrap_or("").to_owned();
    Ok(HitSample {
        normal_set,
        addition_set,
        index,
        volume,
        filename: Some(filename),
    })
}

fn read_hitobject_line(line: &str) -> Result<HitObject, OsuParserError> {
    let mut entries = line.split(',').filter(|&s| !s.is_empty());
    let x: u16 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let y: u16 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let time: f32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let ty: u32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    let hit_sound: u8 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
    println!("Parsing object type {} at {}", ty, time);
    let mut hit_sample = None;
    let specific = if ty & (1 << 0) > 0 {
        // Hit Circle. Nothing extra
        SpecificHitObject::Circle
    } else if ty & (1 << 1) > 0 {
        // Slider
        let curve_data = entries.next().ok_or(OsuParserError::BadFormat)?.split('|');
        let slides: u8 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
        let length: f32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
        let edge_sounds = entries.next().map(|s| s.split('|'));
        let edge_sets = entries.next().map(|s| s.split('|'));
        SpecificHitObject::Slider {}
    } else if ty & (1 << 3) > 0 {
        // Spinner
        let end_time: u32 = entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
        SpecificHitObject::Spinner { end_time }
    } else if ty & (1 << 7) > 0 {
        // Mania Hold
        let mut split = entries.next().ok_or(OsuParserError::BadFormat)?.split(':');
        let end_time: u32 = split.next().ok_or(OsuParserError::BadFormat)?.parse()?;
        let hit_sample_str = split.next().ok_or(OsuParserError::BadFormat)?;
        hit_sample = Some(parse_hit_sample(hit_sample_str)?);
        SpecificHitObject::ManiaHold {}
    } else {
        return Err(OsuParserError::BadFormat);
    };
    let new_combo = ty & (1 << 2) > 0;
    let color_skip = (ty >> 3) & 0b111;
    let split = entries.next();
    if let Some(split) = split {
        hit_sample = Some(parse_hit_sample(split)?);
    }

    Ok(HitObject {
        position: (x, y),
        time: time as u32,
        specific,
        hit_sound,
        hit_sample: hit_sample.unwrap_or(HitSample {
            normal_set: 0,
            addition_set: 0,
            index: 0,
            volume: 100,
            filename: None,
        }),
    })
}
