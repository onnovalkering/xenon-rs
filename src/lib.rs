//! This crate contains Rust bindings to [Xenon]. Xenon is a middleware that provides a uniform
//! interface to various software systems that are used in the area of scientific and high-performance
//! computing. These bindings are based on [gRPC] and require a [Xenon gRPC server] to attach to.
//! Consistency is maintained with [Xenon's Java API].
//!
//! [Xenon]: https://xenon-middleware.github.io
//! [gRPC]: https://grpc.io
//! [Xenon gRPC server]: https://github.com/xenon-middleware/xenon-grpc
//! [Xenon's Java API]: https://xenon-middleware.github.io/xenon/versions/3.1.0/javadoc
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
