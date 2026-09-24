#[derive(Debug, Clone, Default)]
pub struct RandomizerSettings {
    pub seed: Seed,
}

pub type Seed = u64;
