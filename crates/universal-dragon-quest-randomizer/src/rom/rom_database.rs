use super::{Platform, Region, Revision, RomFormat, RomHash, RomId};

include!(concat!(env!("OUT_DIR"), "/rom_database.rs"));

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomDefinition {
    pub id: RomId,
    pub platform: Platform,
    pub region: Region,
    pub format: RomFormat,
    pub revision: Revision,
}

#[derive(Debug)]
pub struct RomEntry {
    pub hash: RomHash,
    pub definition: RomDefinition,
}

pub struct RomDatabase {
    entries: &'static [RomEntry],
}

impl RomDatabase {
    pub fn lookup(&self, hash: &RomHash) -> Option<&RomDefinition> {
        self.entries
            .binary_search_by(|entry| entry.hash.as_bytes().cmp(hash.as_bytes()))
            .ok()
            .map(|index| &self.entries[index].definition)
    }
}
