mod encounters;
mod general;
mod seed;
mod shops;
mod skills;
mod synthesis;

pub use encounters::*;
pub use general::*;
pub use seed::*;
pub use shops::*;
pub use skills::*;
pub use synthesis::*;

#[derive(Debug, Clone, Default)]
pub struct RandomizerConfig {
    pub seed: Seed,

    pub encounters: EncountersConfig,
    pub general: GeneralConfig,
    pub shops: ShopsConfig,
    pub skills: SkillsConfig,
    pub synthesis: SynthesisConfig,
}
