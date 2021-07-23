#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

pub mod compute;
pub mod credentials;
pub mod storage;

#[allow(clippy::all)]
mod xenon {
    tonic::include_proto!("xenon");
}
