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
    ///
    /// 
    /// 
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

    ///
    /// 
    /// 
    pub fn get_attributes(
        &self,
        path: FileSystemPath,
    ) -> FResult<FileSystemAttributes> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let attributes = self.client.get_attributes(&request)?;
        let attributes = FileSystemAttributes::new(attributes);

        Ok(attributes)
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

    ///
    /// 
    /// 
    pub fn get_working_directory(
        &self,
    ) -> FResult<FileSystemPath> {
        let directory = self.client.get_working_directory(&self.filesystem)?;
        let directory = FileSystemPath::new(directory.path);

        Ok(directory)
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

    ///
    /// 
    /// 
    pub fn set_permissions(
        &self,
        path: FileSystemPath,
        permissions: Vec<FileSystemPermission>,
    ) -> FResult<()> {
        let mut request = xenon::SetPosixFilePermissionsRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_permissions(permissions.iter().map(|p| { p.proto() }).collect());

        self.client.set_posix_file_permissions(&request)?;

        Ok(())
    }

    ///
    /// 
    /// 
    pub fn set_working_directory(
        &self,
        path: FileSystemPath,
    ) -> FResult<()> {
        let mut request = xenon::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        self.client.set_working_directory(&request)?;

        Ok(())
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
pub struct FileSystemAttributes {
    pub creation_time: u64,
    pub group: String,
    pub is_directory: bool,
    pub is_executable: bool,
    pub is_hidden: bool,
    pub is_other: bool,
    pub is_readable: bool,
    pub is_regular: bool,
    pub is_symbolic_link: bool,
    pub is_writable: bool,
    pub last_access_time: u64,
    pub last_modified_time: u64,
    pub owner: String,
    pub permissions: Vec<FileSystemPermission>,
    pub size: u64,
}

impl FileSystemAttributes {
    ///
    /// 
    /// 
    pub(crate) fn new(
        attributes: xenon::PathAttributes
    ) -> FileSystemAttributes {
        FileSystemAttributes {
            creation_time: attributes.creation_time,
            group: attributes.group,
            is_directory: attributes.is_directory,
            is_executable: attributes.is_executable,
            is_hidden: attributes.is_hidden,
            is_other: attributes.is_other,
            is_readable: attributes.is_readable,
            is_regular: attributes.is_regular,
            is_symbolic_link: attributes.is_symbolic_link,
            is_writable: attributes.is_writable,
            last_access_time: attributes.last_access_time,
            last_modified_time: attributes.last_modified_time,
            owner: attributes.owner,
            permissions: attributes
                .permissions
                .iter()
                .map(|p| { FileSystemPermission::from(p) })
                .collect(),
            size: attributes.size,
        }
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

///
/// 
///
#[derive(Clone, Debug, PartialEq)]
pub enum FileSystemPermission {
    None = 0,
    OwnerRead = 1,
    OwnerWrite = 2,
    OwnerExecute = 3,
    GroupRead = 4,
    GroupWrite = 5,
    GroupExecute = 6,
    OthersRead = 7,
    OthersWrite = 8,
    OthersExecute = 9,
}

impl FileSystemPermission {
    ///
    /// 
    /// 
    pub(crate) fn from(
        permission: &xenon::PosixFilePermission
    ) -> FileSystemPermission {
        use FileSystemPermission::*;
        use xenon::PosixFilePermission::*;

        match permission {
            NONE => None,
            OWNER_READ => OwnerRead,
            OWNER_WRITE => OwnerWrite,
            OWNER_EXECUTE => OwnerExecute,
            GROUP_READ => GroupRead,
            GROUP_WRITE => GroupWrite,
            GROUP_EXECUTE => GroupExecute,
            OTHERS_READ => OthersRead,
            OTHERS_WRITE => OthersWrite,
            OTHERS_EXECUTE => OthersExecute,
        }
    }

    ///
    /// 
    /// 
    pub fn proto(
        &self,
    ) -> xenon::PosixFilePermission {
        use FileSystemPermission::*;
        use xenon::PosixFilePermission::*;

        match self {
            None => NONE,
            OwnerRead => OWNER_READ,
            OwnerWrite => OWNER_WRITE,
            OwnerExecute => OWNER_EXECUTE,
            GroupRead => GROUP_READ,
            GroupWrite => GROUP_WRITE,
            GroupExecute => GROUP_EXECUTE,
            OthersRead => OTHERS_READ,
            OthersWrite => OTHERS_WRITE,
            OthersExecute => OTHERS_EXECUTE,
        }
    }
}