mod encounters;
mod seed;

pub use encounters::*;
pub use seed::*;

#[derive(Debug, Clone, Default)]
pub struct RandomizerConfig {
    pub seed: Seed,

    pub encounters: EncountersConfig,
}
