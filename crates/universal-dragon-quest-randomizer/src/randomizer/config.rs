#[derive(Debug, Clone, Default)]
pub struct RandomizerConfig {
    pub seed: Seed,
}

pub type Seed = u64;
