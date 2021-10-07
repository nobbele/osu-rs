use osu_parser::load_file;

#[test]
pub fn parse_file() {
    load_file("cYsmix feat. Emmy - Tear Rain (jonathanlfj) [Insane].osu").unwrap();
}
