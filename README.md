# Xenon
[![Build Status](https://github.com/onnovalkering/xenon-rs/workflows/CI/badge.svg)](https://github.com/onnovalkering/xenon-rs/actions)
[![License: MIT](https://img.shields.io/github/license/onnovalkering/xenon-rs.svg)](https://github.com/onnovalkering/xenon-rs/blob/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/onnovalkering/xenon-rs/badge.svg)](https://coveralls.io/github/onnovalkering/xenon-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/xenon-rs)](https://crates.io/crates/xenon-rs)

This crate contains Rust bindings to the Xenon middleware (gRPC).

[Documentation](https://docs.rs/crate/xenon-rs)

## Usage
The interface mimicks that of [PyXenon](https://pyxenon.readthedocs.io) as closely as possible.

```rust
use anyhow::Result;
use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use xenon::credentials::Credential;
use xenon::storage::{FileSystem, FileSystemPath};

fn main() -> Result<()> {
  // Prepare the gRPC channel
  let env = Arc::new(EnvBuilder::new().build());
  let channel = ChannelBuilder::new(env).connect("localhost:50051"); 

  // Connect to the remote server
  let credential = Credential::new_password(String::from("user"), String::from("password"));
  let properties = HashMap::new();
  let filesystem = FileSystem::create(
      String::from("sftp"),
      channel,
      credential,
      String::from("remote-server:22"),
      properties,
  )?;

  // Create a new file
  let new_file = FileSystemPath::new(PathBuf::from("example.txt"));
  filesystem.create_file(&new_file)
}
```

## gRPC Interface
To (re)generate the gRPC interface from the Xenon's `.proto` file:

1. Install the protobuf compiler:
```
$ ./scripts/install-protoc.sh
$ cargo install protobuf-codegen
```

2. Install the gRPC compiler:
```
$ cargo install grpcio-compiler
```

3. Generate the sources:
```
$ cd src/proto
$ GRPC_RUST_PLUGIN=$(which grpc_rust_plugin)
$ protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=$GRPC_RUST_PLUGIN xenon.proto
```
