mod seed;

pub use seed::*;

#[derive(Debug, Clone, Default)]
pub struct RandomizerConfig {
    pub seed: Seed,
}

