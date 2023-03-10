use serde::{Deserialize, Serialize};

use diffusers::schedulers::ddim::{DDIMScheduler, DDIMSchedulerConfig};

use crate::models::DiffuserScheduler;

pub use self::ddim::DDIMScheduler;

mod ddim;
mod der;

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum DiffuserScheduler {
    DDIM(Box<DDIMScheduler>),
}

impl Default for DiffuserScheduler {
    fn default() -> Self {
        Self::DDIM(Box::new(DDIMScheduler::default()))
    }
}

impl Default for DDIMScheduler {
    fn default() -> Self {
        Self {}
    }
}
