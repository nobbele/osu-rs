use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash, num_enum::TryFromPrimitive)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u16)]
pub enum MapMetaType {
    Title = 0,
    Artist = 1,
    Creator = 2,
    Version = 3,
    Source = 4,
    Tags = 5,
    VideoDataOffset = 6,
    VideoDataLength = 7,
    VideoHash = 8,
    BeatmapSetId = 9,
    Genre = 10,
    Language = 11,
    TitleUnicode = 12,
    ArtistUnicode = 13,
    Unknown = 9999,
    Difficulty = 1000,
    PreviewTime = 10001,
    ArtistFullName = 10002,
    ArtistTwitter = 10003,
    SourceUnicode = 10004,
    ArtistUrl = 10005,
    Revision = 10006,
    PackId = 10007,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PackageFile {
    pub length: u32,
    pub hash: [u8; 16],
    //pub creation_time: ??,
    //pub modification_time: ??,
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BeatmapPackage {
    pub metadata: HashMap<MapMetaType, String>,
    pub metadata_hash: [u8; 16],

    pub difficulties: HashMap<String, u32>,
    pub files: HashMap<String, PackageFile>,
}
