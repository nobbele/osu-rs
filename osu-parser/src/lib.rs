use error::{OsuParserError, OsuParserResult};
use osu_types::{BeatmapFileSection, Countdown, Mode, SampleSet};

pub mod error;

#[derive(Default)]
struct InParseGeneral<'a> {
    audio_file_name: Option<&'a str>,
    audio_lead_in: Option<u32>,
    preview_time: Option<u32>,
    countdown: Option<Countdown>,
    sample_set: Option<SampleSet>,
    stack_leniency: Option<f32>,
    mode: Option<Mode>,
    letterbox_in_breaks: Option<bool>,
    widescreen_storyboard: Option<bool>,
}

#[derive(Default)]
struct InParseEditor {
    bookmarks: Vec<u32>,
    distance_spacing: Option<f32>,
    beat_divisor: Option<u8>,
    grid_size: Option<u8>,
}

pub fn split_key_value(line: &str) -> Option<(&str, &str)> {
    let (key, value) = line.split_once(':')?;
    Some((key.trim(), value.trim()))
}

pub fn load_file(path: impl AsRef<std::path::Path>) -> OsuParserResult<()> {
    let content = std::fs::read_to_string(path).unwrap();
    load_content(&content)
}

pub fn load_content(content: &str) -> OsuParserResult<()> {
    let mut lines = content.lines().filter(|&line| !line.trim().is_empty());

    let version_string: &str = lines
        .next()
        .ok_or(OsuParserError::BadFormat)?
        .trim_start_matches("osu file format v");
    let version: u8 = version_string.parse()?;

    if version < 12 {
        return Err(OsuParserError::VersionTooOld(version));
    } else if version > 12 {
        return Err(OsuParserError::VersionTooNew(version));
    }

    let mut current_section: Option<BeatmapFileSection> = None;
    let mut general = InParseGeneral::default();
    let mut editor = InParseEditor::default();

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
                    _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
                }
            }
            BeatmapFileSection::Editor => {
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
            }
            BeatmapFileSection::Metadata => {
                let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
                match key {
                    _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
                }
            }
            BeatmapFileSection::Difficulty => {
                let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
                match key {
                    _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
                }
            }
            BeatmapFileSection::Colours => {
                let (key, value) = split_key_value(line).ok_or(OsuParserError::BadFormat)?;
                match key {
                    _ => return Err(OsuParserError::InvalidKey(key.to_owned())),
                }
            }
            BeatmapFileSection::Events => todo!(),
            BeatmapFileSection::TimingPoints => todo!(),
            BeatmapFileSection::HitObjects => todo!(),
        }
    }

    Ok(())
}
