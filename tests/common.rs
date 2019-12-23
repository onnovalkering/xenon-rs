use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use xenon_rs::credentials::{Credential, PasswordCredential};
use xenon_rs::storage::FileSystem;

type FResult<T> = Result<T, failure::Error>;
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
pub fn create_sftp_filesystem() -> FResult<FileSystem> {
    let channel = build_channel();
    let credential = new_credential();

    let mut properties = Map::<String>::new();
    properties.insert(String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking"), String::from("false"));
    
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
/// 
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
    PasswordCredential::new(
        String::from("xenon"), 
        String::from("javagat"),
    )
}