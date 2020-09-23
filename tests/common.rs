use anyhow::Result;
use log::LevelFilter;
use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use xenon_rs::compute::Scheduler;
use xenon_rs::credentials::Credential;
use xenon_rs::storage::FileSystem;

type Map<T> = std::collections::HashMap<String, T>;

///
///
///
pub fn build_channel() -> Channel {
    let env = Arc::new(EnvBuilder::new().build());
    ChannelBuilder::new(env).connect("localhost:50051")
}

///
///
///
#[allow(dead_code)] 
pub fn create_sftp_filesystem() -> Result<FileSystem> {
    let channel = build_channel();
    let credential = new_credential();

    let mut properties = Map::<String>::new();
    properties.insert(
        String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking"),
        String::from("false"),
    );

    Ok(FileSystem::create(
        String::from("sftp"),
        channel,
        credential,
        String::from("slurm:22"),
        properties,
    )?)
}

///
///
#[allow(dead_code)]
pub fn create_slurm_scheduler() -> Result<Scheduler> {
    let channel = build_channel();
    let credential = new_credential();

    let mut properties = Map::<String>::new();
    properties.insert(
        String::from("xenon.adaptors.schedulers.ssh.strictHostKeyChecking"),
        String::from("false"),
    );

    Ok(Scheduler::create(
        String::from("slurm"),
        channel,
        credential,
        String::from("ssh://slurm:22"),
        properties,
    )?)
}

///
///
///
#[allow(dead_code)]
pub fn get_slurmjob_file() -> Vec<u8> {
    let slurmjob = concat!(
        "#!/bin/bash\n",
        "#SBATCH --job-name test-slurm\n",
        "#SBATCH --output test-slurm.out\n",
        "#SBATCH --error test-slurm.err\n",
        "#SBATCH --time 0:30:00\n",
        "#SBATCH --partition mypartition\n",
        "#SBATCH --ntasks 1\n",
        "\n",
        "date\n",
        "hostname\n",
        "sleep 15\n",
        "date\n"
    );

    slurmjob.as_bytes().to_vec()
}

///
///
///
pub fn new_credential() -> Credential {
    Credential::new_password(String::from("xenon"), String::from("javagat"))
}

///
/// 
/// 
#[allow(dead_code)] 
pub fn show_log() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .init();
}

///
/// 
/// 
#[allow(dead_code)] 
pub fn hide_log() {
    let _ = env_logger::builder()
        .filter_level(LevelFilter::Error)
        .init();
}


