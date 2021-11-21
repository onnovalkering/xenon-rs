# Xenon for Rust
[![Build Status](https://github.com/onnovalkering/xenon-rs/workflows/CI/badge.svg)](https://github.com/onnovalkering/xenon-rs/actions)
[![License: MIT](https://img.shields.io/github/license/onnovalkering/xenon-rs.svg)](https://github.com/onnovalkering/xenon-rs/blob/master/LICENSE)
[![Coverage Status](https://coveralls.io/repos/github/onnovalkering/xenon-rs/badge.svg)](https://coveralls.io/github/onnovalkering/xenon-rs?branch=master)
[![Crates.io](https://img.shields.io/crates/v/xenon-rs)](https://crates.io/crates/xenon-rs)

This crate contains Rust bindings to [Xenon](https://xenon-middleware.github.io). Xenon is a middleware that provides a uniform interface to various software systems that are used in the area of scientific and high-performance computing. These bindings are based on [gRPC](https://grpc.io) and require a [Xenon gRPC server](https://github.com/xenon-middleware/xenon-grpc) to attach to. Consistency is maintained with [Xenon's Java API](https://xenon-middleware.github.io/xenon/versions/3.1.0/javadoc).

[Documentation](https://docs.rs/xenon-rs/latest/xenon)

## Usage
Compute and storage operations can be performed with instances of the `Scheduler` and `FileSystem` structs respectivly. Xenon properties, specific to the used adaptor, can be passed to configure the instances (see [here](https://github.com/onnovalkering/xenon-rs/blob/master/examples/create_file.rs)).

Two types of credentials are supported: regular username/password combinations and (SSH) certificates.

```rust
use xenon::credentials:Credential;
use xenon::compute::Scheduler;
use xenon::storage::FileSystem;

let xenon_endpoint = "http://localhost:50051";
let xenon_properties = None;

let slurm_ssh_host = "remote-server:22";
let slurm_ssh_host_cred = Credential::new_password(
  "username",
  "password",
)

let mut scheduler = Scheduler::create(
  "slurm", 
  slurm_ssh_host,
  slurm_ssh_host_cred,
  xenon_endpoint,
  xenon_properties,
);

let mut filesystem = FileSystem::create(
  "sftp",
  slurm_ssh_host,
  slurm_ssh_host_red,
  xenon_endpoint,
  xenon_properties,
);
```

### Compute
A selection of compute operations:

| Method                   | Description |
|------------------------|-------------|
| [`cancel_job`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.cancel_job) | Cancel a job. |
| [`get_job_status`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.get_job_status) | Get the status of a job. |
| [`get_job_statuses`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.get_job_statuses) | Get the status of multiple jobs. |
| [`get_jobs`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.get_jobs) | Get IDs of all active jobs. |
| [`get_queue_names`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.get_queue_names) | Get the names of the available queues. |
| [`get_queue_status`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.get_queue_status) | Get the status of a queue. |
| [`get_queue_statuses`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.get_queue_statuses) | Get the status of all queues. |
| [`submit_batch_job`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.submit_batch_job) | Submit a batch job. |
| [`wait_until_done`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.wait_until_done) | Wait until a job is done or until it times out. |
| [`wait_until_running`](https://docs.rs/xenon-rs/0.4.2/xenon/compute/struct.Scheduler.html#method.wait_until_running) | Wait until a job is running or until it times out. |

### Storage
A selection of storage operations:

| Method                 | Description |
|------------------------|-------------|
| [`append_to_file`](https://docs.rs/xenon-rs/latest/xenon/storage/struct.FileSystem.html#method.append_to_file) | Append bytes to a file.  |
| [`copy`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.copy) | Copy a file. |
| [`create_directories`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.create_directories) | Create one or more new directories. |
| [`create_directory`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.create_directory) | Create a new directory. |
| [`create_file`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.create_file) | Create a new file. |
| [`create_symbolic_link`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.create_symbolic_link) | Create a symbolic link. |
| [`delete`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.delete) | Delete a file. |
| [`exists`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.exists) | Check if a file exists. |
| [`read_from_file`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.read_from_file) | Read bytes from a file. |
| [`read_symbolic_link`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.read_symbolic_link) | Read the target of a symbolic link. |
| [`rename`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.rename) | Rename a file. |
| [`set_permissions`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.set_permissions) | Set permissions of a file or directory. |
| [`write_to_file`](https://docs.rs/xenon-rs/0.4.2/xenon/storage/struct.FileSystem.html#method.write_to_file) | Write bytes to a file. |

## Examples
Examples of common compute operations:

```rust
use xenon::compute::JobDescription;

// Create a job description.
let job_description = JobDescription {
    executable: Some(String::from("echo")),
    arguments: Some(vec![String::from("Hello, world!")]),
    ..Default::default()
};

// Submit a batch job.
let job = scheduler.submit_batch_job(job_description).await?;

// Retreive the status of the job.
let job_status = scheduler.get_job_status(job).await?;
println!("Job name: {}", job_status.name);

// Cancel the job, if it not already finished.
let job_status = scheduler.cancel_job(job).await?;
```

Examples of common storage operations:

```rust
// Create a new file, if it not already exists.
let example_file = "./example.txt";
if !filesystem.exists(example_file).await? {
    filesystem.create_file(example_file).await?;
}

// Append some text to the file.
let text = String::from("Hello, world!\n");
filesystem.append_to_file(text, example_file).await?;

// Read the contents of the file as text.
let bytes = filesystem.read_from_file(example_file).await?;
let text = String::from_utf8(bytes)?;

// Delete the file.
filesystem.delete(example_file, false).await?;
```

See the [examples](https://github.com/onnovalkering/xenon-rs/tree/master/examples) directory for more examples.
