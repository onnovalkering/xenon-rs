use crate::credentials::{CertificateCredential, Credential, PasswordCredential};
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
}

impl FileSystem {
    pub fn append_to_file(
        &self,
        buffer: Vec<u8>,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::AppendToFileRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_buffer(buffer);

        let (mut sink, mut receiver) = self.client.append_to_file()?;
        sink = sink.send((request, WriteFlags::default())).wait()?;

        future::poll_fn(|| sink.close()).wait()?;
        future::poll_fn(|| receiver.poll()).wait()?;

        Ok(())
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
        request.set_location(location);
        request.set_properties(properties);    
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
            client
        })
    }

    ///
    /// 
    /// 
    pub fn create_directories(
        &self,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        self.client.create_directories(&request)?;

        Ok(())
    }

    ///
    /// 
    /// 
    pub fn create_directory(        
        &self,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        self.client.create_directory(&request)?;

        Ok(())
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

    ///
    /// 
    /// 
    pub fn create_symbolic_link(
        &self,
        link: FileSystemPath,
        target: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::CreateSymbolicLinkRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_link(link.proto());
        request.set_target(target.proto());

        self.client.create_symbolic_link(&request)?;

        Ok(())
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

    ///
    /// 
    /// 
    pub fn get_fs_credential(
        &self
    ) -> FResult<Option<Credential>> {
        let response = self.client.get_credential(&self.filesystem)?;
        if let Some(one_of_credential) = response.credential {
            use xenon::GetCredentialResponse_oneof_credential::*;

            match one_of_credential {
                certificate_credential(credential) => {
                    Ok(Some(CertificateCredential::new(
                        credential.certfile, 
                        credential.username, 
                        credential.passphrase)))
                },
                password_credential(credential) => {
                    Ok(Some(PasswordCredential::new(
                        credential.username, 
                        credential.password)))
                },
                _ => unreachable!(),
            }
        } else {
            Ok(None)
        }
    }

    ///
    /// 
    /// 
    pub fn get_fs_location(
        &self
    ) -> FResult<String> {
        let response = self.client.get_location(&self.filesystem)?;

        Ok(response.location)
    }

    ///
    /// 
    /// 
    pub fn get_fs_separator(        
        &self
    ) -> FResult<String> {
        let response = self.client.get_path_separator(&self.filesystem)?;

        Ok(response.separator)
    }

    ///
    /// 
    /// 
    pub fn get_fs_properties(        
        &self
    ) -> FResult<Map<String>> {
        let response = self.client.get_properties(&self.filesystem)?;

        Ok(response.properties)
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

    ///
    /// 
    /// 
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

    ///
    /// 
    /// 
    pub fn read_symbolic_link(
        &self,
        path: FileSystemPath,
    ) -> FResult<FileSystemPath> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let target = self.client.read_symbolic_link(&request)?;
        let target = FileSystemPath::new(target.path);

        Ok(target)
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
#[derive(Clone, Debug, PartialEq)]
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