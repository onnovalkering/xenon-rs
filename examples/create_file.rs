use anyhow::Result;
use std::collections::HashMap;
use xenon::credentials::Credential;
use xenon::storage::FileSystem;

// Run `docker-compose up --detach` before running this example.

#[tokio::main]
async fn main() -> Result<()> {
    let location = "slurm:22";
    let credential = Credential::new_password("xenon", "javagat");
    let xenon_endpoint = "http://localhost:50051";

    // Disable strict host key checking (only for development setups).
    let mut properties = HashMap::new();
    properties.insert(
        String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking"),
        String::from("false"),
    );

    // Create a new connection to a remote filesystem.
    let mut filesystem = FileSystem::create("sftp", location, credential, xenon_endpoint, Some(properties)).await?;

    // Create a new file, if it not already exists.
    let example_file = "./example.txt";
    if !filesystem.exists(example_file).await? {
        filesystem.create_file(example_file).await?;
    }

    // Append some text to the file.
    let text = String::from("Hello, world!\n");
    filesystem.append_to_file(text, example_file).await?;

    // Close the connection to the remote filesystem.
    filesystem.close().await?;

    Ok(())
}
