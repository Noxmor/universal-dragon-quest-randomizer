use crate::rom::RomId;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to read ROM: {0}")]
    Io(#[from] std::io::Error),

    #[error("unknown ROM")]
    UnknownRom,

    #[error("ambiguous ROM detected as {}", format_ambiguous_rom(.0))]
    AmbiguousRom(Vec<RomId>),

    #[error("ROM is empty")]
    Empty,
}

fn format_ambiguous_rom(ids: &[RomId]) -> String {
    match ids {
        [] => String::new(),
        [id] => format!("\"{id}\""),
        [ids @ .., last] => {
            let ids = ids
                .iter()
                .map(|id| format!("\"{id}\""))
                .collect::<Vec<_>>()
                .join(", ");

            format!("{ids} and \"{last}\"")
        }
    }
}
