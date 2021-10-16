use std::convert::TryInto;

use byteorder::WriteBytesExt;

pub struct FastRandom {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
    // No need for bitBufferIdx since it's only used for NextBool it seems
}

impl FastRandom {
    pub fn new(seed: u32) -> Self {
        FastRandom {
            x: seed,
            y: 842502087,
            z: 3579807591,
            w: 273326509,
        }
    }

    pub fn next_bytes<const N: usize>(&mut self) -> [u8; N] {
        let FastRandom {
            mut x,
            mut y,
            mut z,
            mut w,
        } = *self;
        // TODO Make this non-alloc by using array rather than vec
        // TODO I don't know how to implement Write for [u8; N]
        let mut p = Vec::with_capacity(N);
        let mut t: u32;

        while p.len() < N - 3 {
            t = x ^ (x << 11);
            x = y;
            y = z;
            z = w;
            w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));

            p.write_u32::<byteorder::LE>(w).unwrap();
        }

        if p.len() < N {
            t = x ^ (x << 11);
            x = y;
            y = z;
            z = w;
            w = (w ^ (w >> 19)) ^ (t ^ (t >> 8));
            // If buffer is not a multiple of 4, we will need to write the remaining 1-3 bytes to fill it.
            p.write_u8((w & 0x000000FF) as u8).unwrap();
            if p.len() < N {
                p.write_u8((w & 0x0000FF00) as u8).unwrap();
                if p.len() < N {
                    p.write_u8((w & 0x00FF0000) as u8).unwrap();
                    if p.len() < N {
                        p.write_u8((w & 0xFF000000) as u8).unwrap();
                    }
                }
            }
        }

        *self = FastRandom { x, y, z, w };

        p.try_into().unwrap()
    }
}
