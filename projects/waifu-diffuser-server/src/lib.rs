#![feature(once_cell)]
#![feature(async_closure)]

pub use crate::{
    models::text2image::StableDiffusionWorker,
    session::{WaifuDiffuserServer, WaifuDiffuserSession, GLOBAL_RUNNER},
};

mod models;
mod session;
mod utils;
