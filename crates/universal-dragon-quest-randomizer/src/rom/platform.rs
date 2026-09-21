use super::RomFormat;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum Platform {
    #[default]
    Unknown,
}

impl From<RomFormat> for Platform {
    fn from(format: RomFormat) -> Self {
        match format {
            RomFormat::Unknown => Platform::Unknown,
        }
    }
}
