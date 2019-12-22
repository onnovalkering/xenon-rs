pub mod credentials;
#[path = "proto/xenon.rs"]
mod xenon;
#[path = "proto/xenon_grpc.rs"]
mod xenon_grpc;

#[cfg(test)]
mod tests {
    use crate::xenon::{CreateFileSystemRequest, PasswordCredential, Path, WriteToFileRequest};
    use crate::xenon_grpc::FileSystemServiceClient;
    use futures::{future, Future, Sink};
    use grpcio::{ChannelBuilder, EnvBuilder, WriteFlags};
    use std::collections::HashMap;
    use std::sync::Arc;
    
    #[test]
    fn it_works() {
        let env = Arc::new(EnvBuilder::new().build());
        let channel = ChannelBuilder::new(env).connect("localhost:50051");
        let client = FileSystemServiceClient::new(channel);

        // Password Credentials
        let mut credentials = PasswordCredential::new();
        credentials.set_username(String::from("xenon"));
        credentials.set_password(String::from("javagat"));

        // Create FileSystem
        let mut create_req = CreateFileSystemRequest::new();
        create_req.set_adaptor(String::from("sftp"));
        create_req.set_location(String::from("localhost:10022"));
        create_req.set_password_credential(credentials);
        let mut create_req_properties = HashMap::<String, String>::new();
        create_req_properties.insert(String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking"), String::from("false"));
        create_req.set_properties(create_req_properties);

        let fs = client.create(&create_req).unwrap();

        // Write to file
        let mut path = Path::new();
        path.set_path(String::from("hello.sh"));

        let script = String::from("hello");
        let buffer = script.as_bytes().to_vec();

        let mut write_req = WriteToFileRequest::new();
        write_req.set_filesystem(fs.clone());
        write_req.set_path(path);
        write_req.set_buffer(buffer);

        let (mut sink, _receiver) = client.write_to_file().unwrap();
        sink = sink.send((write_req, WriteFlags::default())).wait().unwrap();

        future::poll_fn(|| sink.close()).wait().unwrap();
    }
}
