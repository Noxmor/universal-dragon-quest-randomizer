mod error;
mod platform;
mod region;
mod revision;
mod rom_database;
mod rom_format;
mod rom_hash;
mod rom_id;

pub use error::*;
pub use platform::*;
pub use region::*;
pub use revision::*;
pub use rom_database::*;
pub use rom_format::*;
pub use rom_hash::*;
pub use rom_id::*;

use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Rom {
    path: PathBuf,
    pub id: RomId,
    pub _platform: Platform,
    pub region: Region,
    pub _format: RomFormat,
    pub revision: Revision,
}

impl Rom {
    pub fn new(path: impl AsRef<Path>, definition: &RomDefinition) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            id: definition.id,
            _platform: definition.platform,
            region: definition.region,
            _format: definition.format,
            revision: definition.revision,
        }
    }

    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let data = fs::read(&path)?;

        if data.is_empty() {
            return Err(Error::Empty);
        }

        let hash = RomHash::from_bytes(&data);

        // TODO: Fall back to structural ROM detection if the hash lookup fails.
        let definition = ROM_DATABASE.lookup(&hash).ok_or(Error::UnknownRom)?;

        Ok(Rom::new(path, definition))
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
