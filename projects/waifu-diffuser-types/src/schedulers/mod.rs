use serde::{Deserialize, Serialize};

use diffusers::schedulers::ddim::{DDIMScheduler, DDIMSchedulerConfig};

use crate::models::DiffuserScheduler;

pub use self::ddim::DDIMScheduler;

mod ddim;
mod der;

pub struct EulerScheduler {}

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type")]
pub enum DiffuserScheduler {
    Euler(Box<EulerScheduler>),
    DDIM(Box<DDIMScheduler>),
}

#[derive(Clone, Debug, Serialize)]
pub enum DiffuserSchedulerKind {
    Euler,
    DDIM
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
