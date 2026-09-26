use std::fs;
use std::path::{Path, PathBuf};

use super::{Error, ROM_DATABASE, Rom, RomHash};

pub struct RomLoader {
    path: PathBuf,
}

impl RomLoader {
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            path: path.as_ref().to_owned(),
        }
    }

    pub fn load(&mut self) -> Result<Rom, Error> {
        let data = fs::read(&self.path)?;

        if data.is_empty() {
            return Err(Error::Empty);
        }

        let hash = RomHash::from_bytes(&data);

        // TODO: Fall back to structural ROM detection if the hash lookup fails.
        let definition = ROM_DATABASE.lookup(&hash).ok_or(Error::UnknownRom)?;

        Ok(Rom::new(self.path.clone(), definition))
    }
}
