#![allow(unused)]

use error::*;
use in_parse_types::*;
use osu_types::{
    BeatmapDifficulty, BeatmapFileSection, BeatmapGeneralData, BeatmapInfo, BeatmapMetadata,
    ComboColour, Countdown, Event, HitObject, Mode, SampleSet, TimingPoint,
};

pub mod error;
mod in_parse_types;

// TODO make these optional at runtime
pub struct Beatmap {
    pub info: BeatmapInfo,
    #[cfg(feature = "read-events")]
    pub events: Vec<Event>,
    #[cfg(feature = "read-colours")]
    pub colours: Vec<ComboColour>,
    #[cfg(feature = "read-timing-points")]
    pub timing_points: Vec<TimingPoint>,
    #[cfg(feature = "read-hit-objects")]
    pub hit_objects: Vec<HitObject>,
}

pub fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((key.trim(), value.trim()))
}

pub fn load_file(path: impl AsRef<std::path::Path>) -> OsuParserResult<Beatmap> {
    let content = std::fs::read_to_string(path).unwrap();
    load_content(&content)
}

pub fn load_content(content: &str) -> OsuParserResult<Beatmap> {
    let mut lines = content
        .lines()
        .filter(|&line| !line.trim().is_empty() && !line.starts_with("//"));

    let version_string: &str = lines
        .next()
        .ok_or(OsuParserError::BadFormat)?
        .trim_start_matches("osu file format v");
    let version: u8 = version_string.parse()?;

    if version < 3 {
        return Err(OsuParserError::VersionTooOld(version));
    } else if version > 14 {
        return Err(OsuParserError::VersionTooNew(version));
    }

    let mut current_section: Option<BeatmapFileSection> = None;
    let mut general = InParseGeneral::default();
    #[cfg(feature = "read-editor")]
    let mut editor = InParseEditor::default();
    let mut metadata = InParseMetadata::default();
    let mut difficulty = InParseDifficulty::default();

    #[cfg(feature = "read-events")]
    let mut events: Vec<Event> = Vec::new();
    #[cfg(feature = "read-timing-points")]
    let mut timing_points: Vec<TimingPoint> = Vec::new();
    #[cfg(feature = "read-colours")]
    let mut colours: Vec<ComboColour> = Vec::new();
    #[cfg(feature = "read-hit-objects")]
    let mut hit_objects: Vec<HitObject> = Vec::new();

    for line in lines {
        if line.starts_with("[") && line.ends_with("]") {
            let section_string = &line[1..line.len() - 1];
            let section = BeatmapFileSection::from_str(section_string)
                .ok_or_else(|| OsuParserError::InvalidSection(section_string.to_owned()))?;
            current_section = Some(section);
            continue;
        }

        let current_section = current_section.ok_or(OsuParserError::DataOutsideSection)?;
        match current_section {
            BeatmapFileSection::General => {
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
                        general.sample_set =
                            Some(SampleSet::from_str(value).ok_or(OsuParserError::BadFormat)?);
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
                    "LetterboxInBreaks" => {
                        general.letterbox_in_breaks = Some(value.parse::<u8>()? != 0)
                    }
                    "WidescreenStoryboard" => {
                        general.widescreen_storyboard = Some(value.parse::<u8>()? != 0)
                    }
                    "AudioHash" => { /* Ignore */ }
                    _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
                }
            }
            BeatmapFileSection::Editor => {
                #[cfg(feature = "read-editor")]
                {
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
                        _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
                    }
                };
            }
            BeatmapFileSection::Metadata => {
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
            }
            BeatmapFileSection::Difficulty => {
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
            }
            BeatmapFileSection::Colours => {
                #[cfg(feature = "read-colours")]
                {
                    let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
                    todo!()
                };
            }
            BeatmapFileSection::Events => {
                #[cfg(feature = "read-events")]
                {
                    let mut entries = line.split(',');
                    let event_type = entries.next().ok_or(OsuParserError::BadFormat)?;
                    let event = match event_type {
                        "0" => {
                            // this is always 0 for some reason
                            entries.next();
                            let filename = entries.next().ok_or(OsuParserError::BadFormat)?;
                            let x_offset = entries.next().ok_or(OsuParserError::BadFormat)?;
                            let y_offset = entries.next().ok_or(OsuParserError::BadFormat)?;
                            Event::Background {
                                filename: filename.to_owned(),
                                offset: (x_offset.parse()?, y_offset.parse()?),
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
                            let start_time: u32 =
                                entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
                            let end_time: u32 =
                                entries.next().ok_or(OsuParserError::BadFormat)?.parse()?;
                            Event::Break(start_time..end_time)
                        }
                        "3" => {
                            // TODO
                            continue;
                        }
                        _ => return Err(OsuParserError::BadFormat),
                    };
                    events.push(event);
                };
            }
            BeatmapFileSection::TimingPoints => {
                #[cfg(feature = "read-timing-points")]
                {
                    todo!()
                };
            }
            BeatmapFileSection::HitObjects => {
                #[cfg(feature = "read-hit-objects")]
                {
                    todo!()
                };
            }
        }
    }

    if cfg!(feature = "read-editor") {
        todo!()
    }

    let title = metadata.title.ok_or(OsuParserError::BadFormat)?;
    let artist = metadata.artist.ok_or(OsuParserError::BadFormat)?;
    let od = difficulty.od.ok_or(OsuParserError::BadFormat)?;
    let beatmap = Beatmap {
        info: BeatmapInfo {
            general_data: BeatmapGeneralData {
                audio_file_name: general
                    .audio_file_name
                    .ok_or(OsuParserError::BadFormat)?
                    .to_owned(),
                audio_lead_in: general.audio_lead_in.unwrap_or(0),
                preview_time: general.preview_time.unwrap_or(0),
                countdown: general.countdown,
                sample_set: general.sample_set.unwrap_or(SampleSet::Normal),
                stack_leniency: general.stack_leniency.unwrap_or(0.5),
                mode: general.mode.unwrap_or(Mode::Osu),
                letterbox_in_breaks: general.letterbox_in_breaks.unwrap_or(false),
                widescreen_storyboard: general.widescreen_storyboard.unwrap_or(false),
            },
            metadata: BeatmapMetadata {
                title: title.to_owned(),
                title_unicode: metadata.title_unicode.unwrap_or(title).to_owned(),
                artist: artist.to_owned(),
                artist_unicode: metadata.artist_unicode.unwrap_or(artist).to_owned(),
                creator: metadata
                    .creator
                    .ok_or(OsuParserError::BadFormat)?
                    .to_owned(),
                version: metadata
                    .version
                    .ok_or(OsuParserError::BadFormat)?
                    .to_owned(),
                source: metadata.source.unwrap_or("").to_owned(),
                tags: metadata.tags.unwrap_or("").to_owned(),
                beatmap_id: metadata.beatmap_id.unwrap_or(-1).to_owned(),
                beatmap_set_id: metadata.beatmap_set_id.unwrap_or(-1).to_owned(),
            },
            difficulty: BeatmapDifficulty {
                hp: difficulty.hp.ok_or(OsuParserError::BadFormat)?,
                cs: difficulty.cs.ok_or(OsuParserError::BadFormat)?,
                od,
                ar: difficulty.ar.unwrap_or(od),
                slider_multiplier: difficulty
                    .slider_multiplier
                    .ok_or(OsuParserError::BadFormat)?,
                slider_tick_rate: difficulty
                    .slider_tick_rate
                    .ok_or(OsuParserError::BadFormat)?,
            },
        },
        #[cfg(feature = "read-events")]
        events,
        #[cfg(feature = "read-colours")]
        colours,
        #[cfg(feature = "read-timing-points")]
        timing_points,
        #[cfg(feature = "read-hit-objects")]
        hit_objects,
    };

    Ok(beatmap)
}
