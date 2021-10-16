use osu_types::osz2::{BeatmapPackage, MapMetaType, PackageFile};
use std::{
    collections::HashMap,
    convert::TryFrom,
    io::{Cursor, Read, Seek},
};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};
use ext::{ReadExt, WriteExt};

use crate::{decryptor_stream::DecryptorStream, fastrandom::FastRandom};

mod decryptor_stream;
mod ext;
mod fastrandom;
mod xxtea;

pub fn generate_key(creator: &str, beatmapset_id: &str) -> [u8; 16] {
    let seed = format!("{}yhxyfjo5{}", creator, beatmapset_id);
    //let seed = format!("{}4390gn8931i{}", creator, beatmapset_id);
    md5::compute(seed.as_bytes()).0
}

pub fn calc_osz2_hash(buffer: &mut [u8], pos: usize, swap: u8) -> [u8; 16] {
    buffer[pos] ^= swap;
    let mut hash = md5::compute(&buffer).0;
    buffer[pos] ^= swap;

    for i in 0..8 {
        // swap but can't be bothered to prove safety.
        let a = hash[i];
        hash[i] = hash[i + 8];
        hash[i + 8] = a;
    }

    hash[5] ^= 0x2d;

    hash
}

/// Returns true if magic byte check passes.
pub fn read_magic(mut reader: impl Read) -> bool {
    let magic: [u8; 3] = reader.read_n().unwrap();
    magic == [0xEC, b'H', b'O']
}

/// First value is the data read, second value is the hash.
pub fn read_metadata(mut reader: impl Read) -> (HashMap<MapMetaType, String>, [u8; 16]) {
    let mut check_cursor = Cursor::new(Vec::new());

    let count = reader.read_i32::<LE>().unwrap();
    check_cursor.write_i32::<LE>(count).unwrap();

    let mut metadata = HashMap::with_capacity(count as usize);

    for _ in 0..count {
        let key = reader.read_u16::<LE>().unwrap();
        let meta_type = MapMetaType::try_from(key).unwrap_or(MapMetaType::Unknown);
        let value = reader.read_net_string().unwrap();

        check_cursor.write_u16::<LE>(key).unwrap();
        check_cursor.write_net_string(&value).unwrap();

        metadata.insert(meta_type, value);
    }

    let hash = calc_osz2_hash(&mut check_cursor.into_inner(), metadata.len() * 3, 0xa7);

    (metadata, hash)
}

pub fn read_difficulty_data(mut reader: impl Read) -> HashMap<String, u32> {
    let map_count = reader.read_i32::<LE>().unwrap();
    let mut difficulties = HashMap::with_capacity(map_count as usize);

    for _ in 0..map_count {
        let name = reader.read_net_string().unwrap();
        let id = reader.read_u32::<LE>().unwrap();

        difficulties.insert(name, id);
    }

    difficulties
}

pub fn read_check_key(mut reader: impl Read, key: &[u32; 4]) {
    let xtea = xtea::XTEA::new(key);

    let mut fastrandom = FastRandom::new(1990);
    let known_plain: [u8; 64] = fastrandom.next_bytes();

    let encrypted_plain: [u8; 64] = reader.read_n().unwrap();
    let mut decrypted_plain = [0; 64];
    xtea.decipher_u8slice::<LE>(&encrypted_plain, &mut decrypted_plain);

    assert_eq!(known_plain, decrypted_plain);
}

pub fn read_data_length(mut reader: impl Read, hash_info: &[u8; 16]) -> usize {
    let mut length = reader.read_i32::<LE>().unwrap();
    for i in (0..16).step_by(2) {
        length -= hash_info[i] as i32 | (hash_info[i + 1] as i32) << 17;
    }
    length as usize
}

pub fn decode_iv(xor_iv: [u8; 16], hash_body: &[u8; 16]) -> [u8; 16] {
    let mut iv = xor_iv;
    for i in 0..iv.len() {
        iv[i] ^= hash_body[i % 16];
    }
    iv
}

// TODO this gives wrong value for some reason
pub fn _body_hash(
    mut reader: impl Read + Seek,
    metadata: &HashMap<MapMetaType, String>,
    data_len: u64,
    pos: u64,
    swap: u8,
) -> [u8; 16] {
    let stream_pos = reader.stream_position().unwrap();
    let remaining_bytes = data_len - stream_pos;

    let mut data = if let Some(_video_offset_s) = metadata.get(&MapMetaType::VideoDataOffset) {
        todo!()
    } else {
        let mut buffer = vec![0; remaining_bytes as usize];
        reader.read_exact(&mut buffer).unwrap();
        buffer
    };

    dbg!(data.len());

    calc_osz2_hash(&mut data, pos as usize, swap)
}

pub fn parse(mut reader: impl Read + Seek, reader_len: u32) -> BeatmapPackage {
    if !read_magic(&mut reader) {
        panic!("Not an osz2 (Magic bytes didn't match)")
    }
    let _version = reader.read_u8().unwrap();
    let xor_iv: [u8; 16] = reader.read_n().unwrap();
    let hash_meta: [u8; 16] = reader.read_n().unwrap();
    let hash_info: [u8; 16] = reader.read_n().unwrap();
    let hash_body: [u8; 16] = reader.read_n().unwrap();

    let (metadata, meta_hash) = read_metadata(&mut reader);
    if meta_hash != hash_meta {
        panic!("Hash in file did not match calculated metadata hash");
    }

    let difficulties = read_difficulty_data(&mut reader);

    let key = generate_key(
        &metadata[&MapMetaType::Creator],
        &metadata[&MapMetaType::BeatmapSetId],
    );
    let key = [
        (&key[0..4]).read_u32::<LE>().unwrap(),
        (&key[4..8]).read_u32::<LE>().unwrap(),
        (&key[8..12]).read_u32::<LE>().unwrap(),
        (&key[12..16]).read_u32::<LE>().unwrap(),
    ];

    read_check_key(&mut reader, &key);

    let _file_info_offset = reader.stream_position().unwrap();
    let data_length = read_data_length(&mut reader, &hash_info);

    let mut file_info_data = vec![0; data_length];
    reader.read_exact(&mut file_info_data).unwrap();

    let data_offset = reader.stream_position().unwrap() as u32;

    /*let body_hash = body_hash(
        &mut reader,
        &metadata,
        file_len,
        (file_len - data_offset) / 2,
        0x9f,
    );
    reader.seek(SeekFrom::Start(data_offset)).unwrap();

    println!("{:?}", body_hash);
    println!("{:?}", hash_body);

    if body_hash != hash_body {
        panic!("Body hash didn't match");
    }*/

    let _iv = decode_iv(xor_iv, &hash_body);

    let count = DecryptorStream::new(&file_info_data[0..4], key)
        .read_i32::<LE>()
        .unwrap();

    let info_hash = calc_osz2_hash(&mut file_info_data, count as usize * 4, 0xd1);
    if info_hash != hash_info {
        panic!("Hash in file did not match calculated file hash");
    }

    let reader = Cursor::new(&file_info_data[4..]);
    let mut reader = DecryptorStream::new(reader, key);

    let mut files = HashMap::new();
    let mut offset_current = reader.read_u32::<LE>().unwrap();

    for i in 0..count {
        let name = reader.read_net_string().unwrap();
        let file_hash: [u8; 16] = reader.read_n().unwrap();
        let _file_date_created = reader.read_u64::<LE>().unwrap();
        let _file_date_modified = reader.read_u64::<LE>().unwrap();

        let offset_next = if i + 1 < count {
            reader.read_u32::<LE>().unwrap()
        } else {
            reader_len - data_offset
        };

        let file_length = offset_next - offset_current;

        if name.ends_with("avi")
            || name.ends_with("flv")
            || name.ends_with("mpg")
            || name.ends_with("wmv")
            || name.ends_with("m4v")
            || name.ends_with("mp4")
        {
            todo!()
        }

        files.insert(
            name,
            PackageFile {
                length: file_length,
                //offset: offset_current,
                hash: file_hash,
            },
        );

        offset_current = offset_next;
    }

    BeatmapPackage {
        metadata,
        metadata_hash: meta_hash,

        difficulties,

        files,
    }
}
