# Xenon
[![Build Status](https://github.com/onnovalkering/xenon-rs/workflows/CI/badge.svg)](https://github.com/onnovalkering/xenon-rs/actions)
[![License: MIT](https://img.shields.io/github/license/onnovalkering/xenon-rs.svg)](https://github.com/onnovalkering/xenon-rs/blob/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/onnovalkering/xenon-rs/badge.svg)](https://coveralls.io/github/onnovalkering/xenon-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/xenon-rs)](https://crates.io/crates/xenon-rs)

This crate contains Rust bindings to the Xenon middleware (gRPC).

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
$ protoc --rust_out=. --grpc_out=. --plugin=protoc-gen-grpc=`which grpc_rust_plugin` xenon.proto
```