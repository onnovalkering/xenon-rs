//! This crate contains Rust bindings to the Xenon middleware (gRPC).
#[macro_use]
extern crate anyhow;
#[macro_use]
extern crate log;

/// Contains the equivalent of Xenon's `Scheduler`.
pub mod compute;
/// Contains the equivalent of Xenon's `Credential`.
pub mod credentials;
/// Contains the equivalent of Xenon's `FileSystem`.
pub mod storage;

#[allow(clippy::all)]
mod xenon {
    tonic::include_proto!("xenon");
}
