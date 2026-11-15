mod encounters;
mod general;
mod seed;

pub use encounters::*;
pub use general::*;
pub use seed::*;

#[derive(Debug, Clone, Default)]
pub struct RandomizerConfig {
    pub seed: Seed,

    pub encounters: EncountersConfig,
    pub general: GeneralConfig,
}
