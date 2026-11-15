mod encounters;
mod general;
mod seed;
mod shops;

pub use encounters::*;
pub use general::*;
pub use seed::*;
pub use shops::*;

#[derive(Debug, Clone, Default)]
pub struct RandomizerConfig {
    pub seed: Seed,

    pub encounters: EncountersConfig,
    pub general: GeneralConfig,
    pub shops: ShopsConfig,
}
