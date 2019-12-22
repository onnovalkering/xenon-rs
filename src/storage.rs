use crate::credentials::Credential;
use crate::xenon;
use crate::xenon::{CreateFileSystemRequest, WriteToFileRequest};
use crate::xenon_grpc::FileSystemServiceClient;
use futures::{future, Future, Sink};
use grpcio::{Channel, WriteFlags};

type FResult<T> = Result<T, failure::Error>;
type Map<T> = std::collections::HashMap<String, T>;

///
/// 
/// 
pub struct FileSystem {
    pub adaptor: String,
    client: FileSystemServiceClient,
    filesystem: xenon::FileSystem,
    pub identifier: String,
    pub location: String,
    pub properties: Map<String>,
}

impl FileSystem {
    ///
    /// 
    /// 
    pub fn create(
        adaptor: String,
        channel: Channel,
        credential: Credential,
        location: String,
        properties: Map<String>,
    ) -> FResult<FileSystem> {
        let client = FileSystemServiceClient::new(channel);

        // Construct create request message.
        let mut request = CreateFileSystemRequest::new();
        request.set_adaptor(adaptor.clone());
        request.set_location(location.clone());
        request.set_properties(properties.clone());    
        match credential {
            Credential::Password(password) => {
                request.set_password_credential(password.proto())
            },
            Credential::Certificate(certificate) => {
                request.set_certificate_credential(certificate.proto())
            },
        }

        let filesystem = client.create(&request)?;
        let identifier = filesystem.id.clone();

        Ok(FileSystem {
            adaptor,
            filesystem,
            identifier,
            location,
            properties,
            client
        })
    }

    ///
    /// 
    /// 
    pub fn write_to_file(
        &self,
        buffer: Vec<u8>,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = WriteToFileRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_buffer(buffer);

        let (mut sink, mut receiver) = self.client.write_to_file()?;
        sink = sink.send((request, WriteFlags::default())).wait()?;

        future::poll_fn(|| sink.close()).wait()?;
        future::poll_fn(|| receiver.poll()).wait()?;

        Ok(())
    }
}

///
/// 
///
pub struct FileSystemPath {
    pub path: String,
}

impl FileSystemPath {
    ///
    /// 
    /// 
    pub fn new(
        path: String,
    ) -> FileSystemPath {
        FileSystemPath {
            path
        }
    }

    ///
    /// 
    ///
    pub(crate) fn proto(
        self
    ) -> xenon::Path {
        let mut path = xenon::Path::new();
        path.set_path(self.path);
        
        path
    }
}