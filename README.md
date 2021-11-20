# Xenon for Rust
[![Build Status](https://github.com/onnovalkering/xenon-rs/workflows/CI/badge.svg)](https://github.com/onnovalkering/xenon-rs/actions)
[![License: MIT](https://img.shields.io/github/license/onnovalkering/xenon-rs.svg)](https://github.com/onnovalkering/xenon-rs/blob/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/onnovalkering/xenon-rs/badge.svg)](https://coveralls.io/github/onnovalkering/xenon-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/xenon-rs)](https://crates.io/crates/xenon-rs)

This crate contains Rust bindings to [Xenon](https://xenon-middleware.github.io). Xenon is a middleware that provides a uniform interface to various software systems that are used in the area of scientific and high-performance computing. These bindings are based on [gRPC](https://grpc.io) and require a [Xenon gRPC server](https://github.com/xenon-middleware/xenon-grpc) to attach to. These bindings are consistent with [Xenon's API](https://xenon-middleware.github.io/xenon/versions/3.0.0/javadoc).

[Documentation](https://docs.rs/xenon-rs/latest/xenon)

## Usage

```rust
use anyhow::Result;
use xenon::credentials::Credential;
use xenon::storage::{FileSystem, FileSystemPath};

#[tokio::main]
async fn main() -> Result<()> {
    let location = "remote-server:22";
    let credential = Credential::new_password("username", "password");
    let xenon_endpoint = "http://localhost:50051";

    // Create a SFTP connection to the remote location.
    let mut filesystem = FileSystem::create(
        "sftp", 
        location, 
        credential, 
        xenon_endpoint, 
        None,
    ).await?;

    // Create a new file, if it not already exists.
    let example_file = "./example.txt";
    if !filesystem.exists(example_file).await? {
        filesystem.create_file(example_file).await?;
    }

    // Append some text to the file.
    let text = String::from("Hello, world!\n");
    filesystem.append_to_file(text, example_file).await?;

    // Close the connection.
    filesystem.close().await?;

    Ok(())
}
```

See the [examples](https://github.com/onnovalkering/xenon-rs/tree/master/examples) directory for more examples.
