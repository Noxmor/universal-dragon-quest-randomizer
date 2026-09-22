use strum::{Display, EnumString};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Display, EnumString)]
pub enum Region {
    #[default]
    Unknown,
    Japan,
    Europe,
    Usa,
}
