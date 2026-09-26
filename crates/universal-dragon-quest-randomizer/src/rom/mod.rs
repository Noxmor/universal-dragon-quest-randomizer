mod error;
mod platform;
mod region;
mod revision;
mod rom_database;
mod rom_format;
mod rom_hash;
mod rom_id;
mod rom_loader;

pub use error::*;
pub use platform::*;
pub use region::*;
pub use revision::*;
pub use rom_database::*;
pub use rom_format::*;
pub use rom_hash::*;
pub use rom_id::*;
pub use rom_loader::*;

use std::path::Path;
use std::path::PathBuf;

pub struct Rom {
    path: PathBuf,
    pub id: RomId,
    pub platform: Platform,
    pub region: Region,
    pub format: RomFormat,
    pub revision: Revision,
}

impl Rom {
    pub fn new(path: impl AsRef<Path>, definition: &RomDefinition) -> Self {
        Self {
            path: path.as_ref().to_owned(),
            id: definition.id,
            platform: definition.platform,
            region: definition.region,
            format: definition.format,
            revision: definition.revision,
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}
