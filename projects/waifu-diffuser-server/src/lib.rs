#![feature(once_cell)]
#![feature(async_closure)]

pub use crate::session::{WaifuDiffuserServer, WaifuDiffuserSession, GLOBAL_RUNNER};

mod models;
mod session;
