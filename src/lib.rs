#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

pub mod compute;
pub mod credentials;
pub mod storage;
#[path = "proto/xenon.rs"]
mod xenon;
#[path = "proto/xenon_grpc.rs"]
mod xenon_grpc;
