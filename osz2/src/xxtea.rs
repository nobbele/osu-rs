const N_MAX: u32 = 16;
const N_MAX_BYTES: u32 = N_MAX * 4;
const DELTA: u32 = 0x9e3779b9;

pub fn decrypt(key: &[u32; 4], buffer: &mut [u8]) {
    let full_word_count = buffer.len() as u32 / N_MAX_BYTES;
    let mut leftover = buffer.len() as u32 % N_MAX_BYTES;

    // Safe because the algorithm switches back to buffer when it can't fit a word.
    let mut word_buffer: &mut [u32] = unsafe {
        std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u32, buffer.len() / 4)
    };
    let na = N_MAX;
    let mut n = N_MAX;

    let rounds = 6 + 52 / n;

    for _word_count in 0..full_word_count {
        let mut z: u32;
        let mut p: u32;
        let mut sum = rounds.wrapping_mul(DELTA);
        let mut y = word_buffer[0];
        loop {
            let e = (sum >> 2) & 3;
            p = na - 1;
            while p > 0 {
                z = word_buffer[(p - 1) as usize];
                word_buffer[p as usize] -= ((z >> 5 ^ y << 2) + (y >> 3 ^ z << 4))
                    ^ ((sum ^ y) + (key[((p & 3) ^ e) as usize] ^ z));
                y = word_buffer[p as usize];

                p -= 1;
            }

            z = word_buffer[(na - 1) as usize];
            word_buffer[0] -= ((z >> 5 ^ y << 2) + (y >> 3 ^ z << 4))
                ^ ((sum ^ y) + (key[((p & 3) ^ e) as usize] ^ z));
            y = word_buffer[0];

            sum = sum.wrapping_sub(DELTA);
            if sum == 0 {
                break;
            }
        }

        word_buffer = &mut word_buffer[N_MAX as usize..];
    }

    if leftover == 0 {
        return;
    }

    n = leftover / 4;
    if n > 1 {
        decrypt_words(word_buffer, n, key);

        leftover -= n * 4;
        if leftover == 0 {
            return;
        }
    }

    let len = buffer.len();
    let byte_buffer = &mut buffer[len - leftover as usize..];
    simple_decrypt_bytes(byte_buffer, key);
}

fn rotate_left(val: u8, n: u8) -> u8 {
    ((val << n) as u32 | ((val as u32) >> (8 - n as u32))) as u8
}

fn simple_decrypt_bytes(buffer: &mut [u8], key: &[u32; 4]) {
    let key_b: &[u8] = bytemuck::cast_slice(key);

    let mut prev_e: u8 = 0;
    for i in 0..buffer.len() {
        let tmp_e = buffer[i];
        buffer[i] = rotate_left(buffer[i], ((!(prev_e as u32)) % 7) as u8);
        buffer[i] ^= rotate_left(
            key_b[15 - i % 16],
            (((prev_e as usize + buffer.len() - i) as u32) % 7) as u8,
        );
        buffer[i] = (((buffer[i] as u32).wrapping_sub((key_b[i % 16] as u32) >> 2)) % 256) as u8;

        prev_e = tmp_e;
    }
}

fn decrypt_words(word_buffer: &mut [u32], n: u32, key: &[u32; 4]) {
    let mut z: u32;
    let mut p: u32;
    let rounds = 6 + 52 / n;
    let mut sum = rounds.wrapping_mul(DELTA);
    let mut y = word_buffer[0];
    loop {
        let e = (sum >> 2) & 3;
        p = n - 1;
        while p > 0 {
            z = word_buffer[(p - 1) as usize];
            word_buffer[p as usize] = word_buffer[p as usize].wrapping_sub(
                (z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4)
                    ^ ((sum ^ y).wrapping_add(key[((p & 3) ^ e) as usize] ^ z)),
            );
            y = word_buffer[p as usize];

            p -= 1;
        }

        z = word_buffer[(n - 1) as usize];
        word_buffer[0] = word_buffer[0].wrapping_sub(
            (z >> 5 ^ y << 2).wrapping_add(y >> 3 ^ z << 4)
                ^ ((sum ^ y).wrapping_add(key[((p & 3) ^ e) as usize] ^ z)),
        );
        y = word_buffer[0];

        sum = sum.wrapping_sub(DELTA);
        if sum == 0 {
            break;
        }
    }
}
