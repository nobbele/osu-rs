use std::io::BufReader;

use osz2::parse;

fn main() {
    let file = std::fs::File::open("cross time.osz2").unwrap();
    let file_len = file.metadata().unwrap().len();
    let reader = BufReader::new(file);
    parse(reader, file_len as u32);
}
