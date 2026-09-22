use strum::{Display, EnumString};

use super::RomFormat;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Display, EnumString)]
pub enum Platform {
    #[default]
    Unknown,

    #[strum(to_string = "NDS")]
    Nds,

    #[strum(to_string = "3DS")]
    ThreeDs,
}

impl From<RomFormat> for Platform {
    fn from(format: RomFormat) -> Self {
        match format {
            RomFormat::Unknown => Platform::Unknown,
            RomFormat::Nds => Platform::Nds,
            RomFormat::ThreeDs => Platform::ThreeDs,
        }
    }
}
