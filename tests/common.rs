use grpcio::{Channel, ChannelBuilder, EnvBuilder};
use std::sync::Arc;
use xenon_rs::credentials::{Credential, PasswordCredential};
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
pub fn create_sftp_filesystem() -> FileSystem {
    let channel = build_channel();
    let credential = new_credential();

    let mut properties = Map::<String>::new();
    properties.insert(String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking"), String::from("false"));
    
    FileSystem::create(
        String::from("sftp"), 
        channel, 
        credential, 
        String::from("localhost:10022"),
        properties,
    ).unwrap()
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