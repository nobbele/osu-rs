use osu_parser::load_file;
use osu_types::{Mode, SampleSet};

#[test]
pub fn parse_very_old() {
    let beatmap = load_file("Kenji Ninuma - DISCOüÜPRINCE (peppy) [Normal].osu").unwrap();

    assert_eq!(beatmap.info.general_data.audio_file_name, "20.mp3");

    assert_eq!(beatmap.info.metadata.title, "DISCO★PRINCE");
    assert_eq!(beatmap.info.metadata.artist, "Kenji Ninuma");
    assert_eq!(beatmap.info.metadata.creator, "peppy");
    assert_eq!(beatmap.info.metadata.version, "Normal");
}

#[test]
pub fn parse_old() {
    let beatmap = load_file("cYsmix feat. Emmy - Tear Rain (jonathanlfj) [Insane].osu").unwrap();

    assert_eq!(beatmap.info.general_data.audio_file_name, "tearrain.mp3");
    assert_eq!(beatmap.info.general_data.audio_lead_in, 1500);
    assert_eq!(beatmap.info.general_data.preview_time, 195852);
    assert_eq!(beatmap.info.general_data.countdown, None);
    assert_eq!(beatmap.info.general_data.sample_set, SampleSet::Soft);
    assert_eq!(beatmap.info.general_data.stack_leniency, 0.5);
    assert_eq!(beatmap.info.general_data.mode, Mode::Osu);
    assert_eq!(beatmap.info.general_data.letterbox_in_breaks, false);
    assert_eq!(beatmap.info.general_data.widescreen_storyboard, false);

    assert_eq!(beatmap.info.metadata.title, "Tear Rain");
    assert_eq!(beatmap.info.metadata.title_unicode, "Tear Rain");
    assert_eq!(beatmap.info.metadata.artist, "cYsmix feat. Emmy");
    assert_eq!(beatmap.info.metadata.artist_unicode, "cYsmix feat. えみぃ");
    assert_eq!(beatmap.info.metadata.creator, "jonathanlfj");
    assert_eq!(beatmap.info.metadata.version, "Insane");
    assert_eq!(beatmap.info.metadata.source, "Touhou");
    assert_eq!(beatmap.info.metadata.tags, "Amateras Records Mizuyosi Radical Destruction 魔法少女達の百年祭 The Centennial Festival for Magical Girls 東方紅魔郷 ～ the Embodiment of Scarlet Devil Extra stage theme monthly beatmapping contest two");
    assert_eq!(beatmap.info.metadata.beatmap_id, 351189);
    assert_eq!(beatmap.info.metadata.beatmap_set_id, 140662);
}

#[test]
pub fn parse_modern() {
    let beatmap = load_file("Nakiri Ayame - Good-bye sengen (Mir) [Extra].osu").unwrap();

    assert_eq!(beatmap.info.general_data.audio_file_name, "audio.mp3");
    assert_eq!(beatmap.info.general_data.audio_lead_in, 0);
    assert_eq!(beatmap.info.general_data.preview_time, 35569);
    assert_eq!(beatmap.info.general_data.countdown, None);
    assert_eq!(beatmap.info.general_data.sample_set, SampleSet::Soft);
    assert_eq!(beatmap.info.general_data.stack_leniency, 0.4);
    assert_eq!(beatmap.info.general_data.mode, Mode::Osu);
    assert_eq!(beatmap.info.general_data.letterbox_in_breaks, false);
    assert_eq!(beatmap.info.general_data.widescreen_storyboard, true);

    assert_eq!(beatmap.info.metadata.title, "Good-bye sengen");
    assert_eq!(beatmap.info.metadata.title_unicode, "グッバイ宣言");
    assert_eq!(beatmap.info.metadata.artist, "Nakiri Ayame");
    assert_eq!(beatmap.info.metadata.artist_unicode, "百鬼あやめ");
    assert_eq!(beatmap.info.metadata.creator, "Mir");
    assert_eq!(beatmap.info.metadata.version, "Extra");
    assert_eq!(beatmap.info.metadata.source, "");
    assert_eq!(beatmap.info.metadata.tags, "ホロライブ hololive vtuber virtual youtuber chinozo japanese pop jpop j-pop さんかくずわり sankaku zuwari cover flower goodbye declaration 引き籠り hikikomori matha -_matha_- petal wanpachi dada deppy deppyforce");
    assert_eq!(beatmap.info.metadata.beatmap_id, 3020125);
    assert_eq!(beatmap.info.metadata.beatmap_set_id, 1471082);
}
