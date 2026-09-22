use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString)]
pub enum RomFormat {
    #[default]
    Unknown,

    #[strum(to_string = "nds")]
    Nds,

    #[strum(to_string = "3ds")]
    ThreeDs,
}
