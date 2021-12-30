use osu_types::BeatmapFileSection;

pub type OsuParserResult<T> = Result<T, (Option<BeatmapFileSection>, OsuParserError)>;

#[derive(thiserror::Error, Debug)]
pub enum OsuParserError {
    #[error("Failed to read file '{0}'")]
    IOError(#[from] std::io::Error),

    #[error("Unsupported file format version v{0}")]
    UnsupportedVersion(u8),

    #[error("Unable to parse file format version '{0}'")]
    VersionParse(String),

    #[error("Version was too old v{0}")]
    VersionTooOld(u8),

    #[error("Version was too new v{0}")]
    VersionTooNew(u8),

    #[error("Invalid section '{0}'")]
    InvalidSection(String),

    #[error("Found data outside a section")]
    DataOutsideSection,

    #[error("Section contained unexpected format")]
    BadFormat,

    #[error("Invalid key '{0}' in section")]
    InvalidKey(String),

    #[error("Unable to parse integer '{0}'")]
    IntegerParse(#[from] std::num::ParseIntError),

    #[error("Unable to parse float '{0}'")]
    FloatParse(#[from] std::num::ParseFloatError),
}
