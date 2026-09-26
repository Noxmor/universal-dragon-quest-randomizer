use crate::rom::{Error, Platform, Region, Revision, RomFormat, RomId};
use rom_reader::RomReader;
use std::path::Path;

mod rom_reader;

#[derive(Debug)]
pub struct Detection {
    pub id: RomId,
    pub platform: Platform,
    pub region: Region,
    pub format: RomFormat,
    pub revision: Revision,
}

pub fn detect(path: impl AsRef<Path>) -> Result<Detection, Error> {
    let _reader = RomReader::open(path)?;

    let mut matches = Vec::new();

    // INFO: All platform/ROM detection functions should be called here.
    // Each platform detector is responsible for determining whether the ROM
    // matches its format and, if so, returning the corresponding ROM.
    // Example:
    //     if let Some(detection) = platform::detect(&mut reader)? {
    //         matches.push(detection);
    //     }

    match matches.len() {
        0 => Err(Error::UnknownRom),
        1 => Ok(matches.pop().unwrap()),
        _ => Err(Error::AmbiguousRom(
            matches.into_iter().map(|detection| detection.id).collect(),
        )),
    }
}
