use crate::credentials::Credential;
use crate::xenon;
use crate::xenon_grpc::FileSystemServiceClient;
use futures::{future, Future, Sink, Stream};
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
    pub fn append_to_file() -> FResult<()> {
        unimplemented!()
    }

    ///
    /// 
    /// 
    pub fn close(
        &self
    ) -> FResult<()> {
        self.client.close(&self.filesystem)?;

        Ok(())
    }

    pub fn copy() -> FResult<()> {
        unimplemented!()
    }

    pub fn copy_cancel() -> FResult<()> {
        unimplemented!()
    }

    pub fn copy_status() -> FResult<()> {
        unimplemented!()
    }

    pub fn copy_wait() -> FResult<()> {
        unimplemented!()
    }

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
        let mut request = xenon::CreateFileSystemRequest::new();
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

    pub fn create_directories() -> FResult<()> {
        unimplemented!()
    }

    pub fn create_directory() -> FResult<()> {
        unimplemented!()
    }

    ///
    /// 
    /// 
    pub fn create_file(
        &self,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        self.client.create_file(&request)?;

        Ok(())
    }

    pub fn create_symbolic_link() -> FResult<()> {
        unimplemented!()
    }

    ///
    /// 
    /// 
    pub fn delete(
        &self,
        path: FileSystemPath,
        recursive: bool,
    ) -> FResult<()> {
        let mut request = xenon::DeleteRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_recursive(recursive);

        self.client.delete(&request)?;

        Ok(())
    }

    ///
    /// 
    /// 
    pub fn exists(
        &self,
        path: FileSystemPath,
    ) -> FResult<bool> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let is = self.client.exists(&request)?;

        Ok(is.value)
    }

    pub fn get_attributes() -> FResult<()> {
        unimplemented!()
    }

    pub fn get_fs_credential() -> FResult<()> {
        unimplemented!()
    }

    pub fn get_fs_location() -> FResult<()> {
        unimplemented!()
    }

    pub fn get_fs_seperator() -> FResult<()> {
        unimplemented!()
    }

    pub fn get_fs_properties() -> FResult<()> {
        unimplemented!()
    }

    pub fn get_working_directory() -> FResult<()> {
        unimplemented!()
    }

    pub fn is_open() -> FResult<bool> {
        unimplemented!()
    }

    pub fn list() -> FResult<()> {
        unimplemented!()
    }

    pub fn rename() -> FResult<()> {
        unimplemented!()
    }

    pub fn read_from_file(
        &self,
        path: FileSystemPath,
    ) -> FResult<Option<Vec<u8>>> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let mut receiver = self.client.read_from_file(&request)?;
        if let Some(file) = future::poll_fn(|| receiver.poll()).wait()? {
            Ok(Some(file.buffer))
        } else {
            Ok(None)
        }
    }

    pub fn read_symbolic_link() -> FResult<()> {
        unimplemented!()
    }

    pub fn set_permissions() -> FResult<()> {
        unimplemented!()
    }

    pub fn set_working_directory() -> FResult<()> {
        unimplemented!()
    }

    ///
    /// 
    /// 
    pub fn write_to_file(
        &self,
        buffer: Vec<u8>,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::WriteToFileRequest::new();
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

impl Drop for FileSystem {
    ///
    /// 
    /// 
    fn drop(&mut self) {
        self.close().unwrap();
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