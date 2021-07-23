# Xenon
[![Build Status](https://github.com/onnovalkering/xenon-rs/workflows/CI/badge.svg)](https://github.com/onnovalkering/xenon-rs/actions)
[![License: MIT](https://img.shields.io/github/license/onnovalkering/xenon-rs.svg)](https://github.com/onnovalkering/xenon-rs/blob/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/onnovalkering/xenon-rs/badge.svg)](https://coveralls.io/github/onnovalkering/xenon-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/xenon-rs)](https://crates.io/crates/xenon-rs)

This crate contains Rust bindings to the Xenon middleware (gRPC).

[Documentation](https://docs.rs/xenon-rs/latest/xenon)

## Usage
The interface mimicks that of [PyXenon](https://pyxenon.readthedocs.io) as closely as possible.

```rust
use anyhow::Result;
use maplit::hashmap;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use xenon::credentials::Credential;
use xenon::storage::{FileSystem, FileSystemPath};

fn main() -> Result<()> {
  // Connect to the remote server
  let credential = Credential::new_password("xenon", "javagat");
  let properties = hashmap!{
      "xenon.adaptors.filesystems.sftp.strictHostKeyChecking" => "false",
  };

  let filesystem = FileSystem::create(
      "sftp",
      "localhost:50051",
      credential,
      "remote-server:22",
      properties,
  )?;

  // Create a new file
  let new_file = FileSystemPath::new("example.txt");
  filesystem.create_file(&new_file)?;
}
```