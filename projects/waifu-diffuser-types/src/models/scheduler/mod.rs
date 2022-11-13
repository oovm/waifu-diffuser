use super::*;
use diffusers::schedulers::ddim::{DDIMScheduler, DDIMSchedulerConfig};

impl Default for DiffuserScheduler {
    fn default() -> Self {
        Self::DDIM(Box::new(DDIMAdvance::default()))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DDIMAdvance {}

impl Default for DDIMAdvance {
    fn default() -> Self {
        Self {}
    }
}

impl DiffuserScheduler {
    pub fn as_scheduler(&self, steps: usize) -> DDIMScheduler {
        match self {
            Self::DDIM(advance) => {
                let mut config = DDIMSchedulerConfig::default();
                config.train_timesteps = steps;
                DDIMScheduler::new(0, config)
            }
        }
    }
}
