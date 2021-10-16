use std::io::{Read, Write};

use byteorder::{ReadBytesExt, WriteBytesExt};

pub trait ReadExt {
    fn read_n<const N: usize>(&mut self) -> std::io::Result<[u8; N]>;
    fn read_net_string(&mut self) -> std::io::Result<String>;
}

impl<T: Read> ReadExt for T {
    fn read_n<const N: usize>(&mut self) -> std::io::Result<[u8; N]> {
        let mut buf = [0; N];
        self.read_exact(&mut buf)?;
        Ok(buf)
    }

    fn read_net_string(&mut self) -> std::io::Result<String> {
        let mut length: usize = 0;
        for _ in 0..4 {
            let b = self.read_u8().unwrap();
            let flag = b >> 7;
            let val = b & 0b01111111;
            length += val as usize;
            if flag == 0 {
                break;
            }
        }
        let mut buf = vec![0; length];
        self.read_exact(&mut buf).unwrap();
        let s = String::from_utf8(buf).unwrap();
        Ok(s)
    }
}

pub trait WriteExt {
    fn write_net_string(&mut self, s: &str) -> std::io::Result<()>;
}

impl<T: Write> WriteExt for T {
    fn write_net_string(&mut self, s: &str) -> std::io::Result<()> {
        let mut len = s.len();
        loop {
            let flag = len >= 0x7F;
            let val = (len & 0x7F) as u8;
            let b = val | if flag { 0b10000000 } else { 0 };
            self.write_u8(b).unwrap();
            if !flag {
                break;
            }
            len >>= 7;
        }
        self.write_all(s.as_bytes()).unwrap();
        Ok(())
    }
}
