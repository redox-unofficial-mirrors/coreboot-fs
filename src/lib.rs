#![no_std]

use core::mem;
use plain;

pub use self::big_endian::Be32;
mod big_endian;

pub use self::file::File;
mod file;

pub use self::fmap::Fmap;
mod fmap;

pub use self::header::Header;
mod header;

pub struct Rom<'a>(pub &'a [u8]);

impl<'a> Rom<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self(data)
    }

    pub fn fmap(&self) -> Option<&Fmap> {
        let mut offset = 0;
        while offset + mem::size_of::<Fmap>() < self.0.len() {
            let fmap: &Fmap = plain::from_bytes(&self.0[offset..]).ok()?;

            if &fmap.signature == b"__FMAP__" {
                return Some(fmap);
            }

            offset += 8;
        }
        None
    }

    pub fn header_offset(&self) -> Option<usize> {
        if self.0.len() < 4 {
            return None;
        }

        // The location is stored as a negative offset from the end of flash.
        let i = self.0.len() - 4;
        let offset = i32::from_le_bytes([self.0[i], self.0[i + 1], self.0[i + 2], self.0[i + 3]]) as isize;
        let header_pos = (self.0.len() as isize + offset) as usize;

        Some(header_pos)
    }

    pub fn header(&self) -> Option<&Header> {
        let offset = self.header_offset()?;

        if offset >= self.0.len() {
            return None;
        }

        let header: &Header = plain::from_bytes(&self.0[offset..]).ok()?;
        if &header.magic == b"ORBC" {
            Some(header)
        } else {
            None
        }
    }

    pub fn file(&self, offset: usize) -> Option<&File> {
        if offset >= self.0.len() {
            return None;
        }

        let file: &File = plain::from_bytes(&self.0[offset..]).ok()?;
        if &file.magic == b"LARCHIVE" {
            Some(file)
        } else {
            None
        }
    }
}
