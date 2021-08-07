use crate::credentials::Credential;
use crate::xenon::{self as x, file_system_service_client::FileSystemServiceClient};
use anyhow::Result;
use futures::StreamExt;
use futures_util::stream;
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use tonic::transport::Channel;
use tonic::Status;

///
///
///
pub struct FileSystem {
    pub adaptor: String,
    client: FileSystemServiceClient<Channel>,
    pub(crate) filesystem: x::FileSystem,
    pub identifier: String,
}

impl FileSystem {
    ///
    ///
    ///
    pub async fn append_to_file<B, P>(
        &mut self,
        buffer: B,
        path: P,
    ) -> Result<()>
    where
        B: Into<Vec<u8>>,
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::AppendToFileRequest {
            buffer: buffer.into(),
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        let request = stream::iter(vec![request]);
        self.client.append_to_file(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn close(&mut self) -> Result<()> {
        if self.is_open().await? {
            debug!("Closing filesystem: {}", self.identifier);

            self.client.close(self.filesystem.clone()).await?;
        }

        Ok(())
    }

    ///
    ///
    ///
    pub async fn cancel(
        &mut self,
        copy_operation: String,
    ) -> Result<CopyStatus> {
        let request = x::CopyOperationRequest {
            filesystem: Some(self.filesystem.clone()),
            copy_operation: Some(x::CopyOperation { id: copy_operation }),
        };

        let response = self.client.cancel(request).await?;
        let response = response.into_inner();

        Ok(CopyStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn copy<P1, P2>(
        &mut self,
        source: P1,
        destination: P2,
        destination_filesystem: Option<FileSystem>,
        recursive: bool,
    ) -> Result<String>
    where
        P1: Into<FileSystemPath>,
        P2: Into<FileSystemPath>,
    {
        let source = source.into();
        let destination = destination.into();

        let destination_filesystem = destination_filesystem
            .map(|f| f.filesystem)
            .unwrap_or_else(|| self.filesystem.clone());

        let request = x::CopyRequest {
            destination: Some(destination.proto()),
            destination_filesystem: Some(destination_filesystem.clone()),
            filesystem: Some(self.filesystem.clone()),
            mode: 0,
            recursive,
            source: Some(source.proto()),
        };

        let response = self.client.copy(request).await?;
        let response = response.into_inner();

        Ok(response.id)
    }

    ///
    ///
    ///
    pub async fn create<S1, S2, S3>(
        adaptor: S1,
        location: S2,
        credential: Credential,
        xenon_endpoint: S3,
        properties: Option<HashMap<String, String>>,
    ) -> Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
        S3: Into<String>,
    {
        let adaptor = adaptor.into();
        let credential = match credential {
            Credential::Password(password) => {
                x::create_file_system_request::Credential::PasswordCredential(password.proto())
            }
            Credential::Certificate(certificate) => {
                x::create_file_system_request::Credential::CertificateCredential(certificate.proto())
            }
        };

        // Construct create request message.
        let request = x::CreateFileSystemRequest {
            adaptor: adaptor.clone(),
            location: location.into(),
            properties: properties.unwrap_or_default(),
            credential: Some(credential),
        };

        let mut client = FileSystemServiceClient::connect(xenon_endpoint.into()).await?;
        let filesystem = client.create(request).await?.into_inner();
        let identifier = filesystem.id.clone();

        Ok(FileSystem {
            adaptor,
            filesystem,
            identifier,
            client,
        })
    }

    ///
    ///
    ///
    pub async fn create_local<S1>(xenon_endpoint: S1) -> Result<Self>
    where
        S1: Into<String>,
    {
        let xenon_endpoint = xenon_endpoint.into();
        let mut client = FileSystemServiceClient::connect(xenon_endpoint.clone()).await?;

        let response = client.local_file_systems(x::Empty {}).await?;
        let response = response.into_inner();

        let identifier = response.filesystems.first().cloned().map(|f| f.id);
        ensure!(identifier.is_some(), "Failed to create local filesystem.");

        Self::restore(identifier.unwrap(), xenon_endpoint).await
    }

    ///
    ///
    ///
    pub async fn create_directories<P>(
        &mut self,
        path: P,
    ) -> Result<()>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.create_directories(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn create_directory<P>(
        &mut self,
        path: P,
    ) -> Result<()>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.create_directory(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn create_file<P>(
        &mut self,
        path: P,
    ) -> Result<()>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.create_file(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn create_symbolic_link<P1, P2>(
        &mut self,
        link: P1,
        target: P2,
    ) -> Result<()>
    where
        P1: Into<FileSystemPath>,
        P2: Into<FileSystemPath>,
    {
        let link = link.into();
        let target = target.into();

        let request = x::CreateSymbolicLinkRequest {
            link: Some(link.proto()),
            target: Some(target.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.create_symbolic_link(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn delete<P>(
        &mut self,
        path: P,
        recursive: bool,
    ) -> Result<()>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::DeleteRequest {
            path: Some(path.proto()),
            recursive,
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.delete(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn exists<P>(
        &mut self,
        path: P,
    ) -> Result<bool>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        let response = self.client.exists(request).await?;
        let response = response.into_inner();

        Ok(response.value)
    }

    ///
    ///
    ///
    pub async fn get_attributes<P>(
        &mut self,
        path: P,
    ) -> Result<FileSystemAttributes>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        let response = self.client.get_attributes(request).await?;
        let response = response.into_inner();

        let attributes = FileSystemAttributes::from(response);
        Ok(attributes)
    }

    ///
    ///
    ///
    pub async fn get_credential(&mut self) -> Result<Option<Credential>> {
        let response = self.client.get_credential(self.filesystem.clone()).await?;
        let response = response.into_inner();

        if let Some(credential) = response.credential {
            use x::get_credential_response::Credential::*;

            match credential {
                CertificateCredential(credential) => Ok(Some(Credential::new_certificate(
                    credential.certfile,
                    credential.username,
                    credential.passphrase,
                ))),
                PasswordCredential(credential) => {
                    Ok(Some(Credential::new_password(credential.username, credential.password)))
                }
                _ => unreachable!(),
            }
        } else {
            Ok(None)
        }
    }

    ///
    ///
    ///
    pub async fn get_adaptor(&mut self) -> Result<String> {
        let response = self.client.get_adaptor_name(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.name)
    }

    ///
    ///
    ///
    pub async fn get_available_adaptors(&mut self) -> Result<Vec<String>> {
        let response = self.client.get_adaptor_names(x::Empty {}).await?;
        let response = response.into_inner();

        Ok(response.name)
    }

    ///
    ///
    ///
    pub async fn get_location(&mut self) -> Result<String> {
        let response = self.client.get_location(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.location)
    }

    ///
    ///
    ///
    pub async fn get_properties(&mut self) -> Result<HashMap<String, String>> {
        let response = self.client.get_properties(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.properties)
    }

    ///
    ///
    ///
    pub async fn get_separator(&mut self) -> Result<String> {
        let response = self.client.get_path_separator(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.separator)
    }

    ///
    ///
    ///
    pub async fn get_status(
        &mut self,
        copy_operation: String,
    ) -> Result<CopyStatus> {
        let request = x::CopyOperationRequest {
            filesystem: Some(self.filesystem.clone()),
            copy_operation: Some(x::CopyOperation { id: copy_operation }),
        };

        let response = self.client.get_status(request).await?;
        let response = response.into_inner();

        Ok(CopyStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn get_working_directory(&mut self) -> Result<FileSystemPath> {
        let response = self.client.get_working_directory(self.filesystem.clone()).await?;
        let response = response.into_inner();

        let directory = FileSystemPath::from(response);
        Ok(directory)
    }

    ///
    ///
    ///
    pub async fn list_filesystems<S1>(xenon_endpoint: S1) -> Result<Vec<String>>
    where
        S1: Into<String>,
    {
        let xenon_endpoint = xenon_endpoint.into();
        let mut client = FileSystemServiceClient::connect(xenon_endpoint.clone()).await?;

        let response = client.list_file_systems(x::Empty {}).await?;
        let response = response.into_inner();

        let identifiers = response.filesystems.into_iter().map(|f| f.id).collect();

        Ok(identifiers)
    }

    ///
    ///
    ///
    pub async fn is_open(&mut self) -> Result<bool> {
        let response = self.client.is_open(self.filesystem.clone()).await;
        let value = if let Ok(response) = response {
            let response = response.into_inner();
            response.value
        } else {
            false
        };

        Ok(value)
    }

    ///
    ///
    ///
    pub async fn list<P>(
        &mut self,
        path: P,
        recursive: bool,
    ) -> Result<Vec<FileSystemAttributes>>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::ListRequest {
            dir: Some(path.proto()),
            recursive,
            filesystem: Some(self.filesystem.clone()),
        };

        let response = self.client.list(request).await?;
        let response = response.into_inner();

        let files: Vec<Result<x::PathAttributes, Status>> = response.collect().await;

        let files = files
            .into_iter()
            .filter(|f| f.is_ok())
            .map(|f| FileSystemAttributes::from(f.unwrap()))
            .collect();

        Ok(files)
    }

    ///
    ///
    ///
    pub async fn read_from_file<P>(
        &mut self,
        path: P,
    ) -> Result<Vec<u8>>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        let response = self.client.read_from_file(request).await?;
        let response = response.into_inner();

        let chunks: Vec<Result<x::ReadFromFileResponse, Status>> = response.collect().await;

        if chunks.iter().any(|c| c.is_err()) {
            bail!("Failed to read file: received at least one faulty chunk.");
        }

        let file: Vec<u8> = chunks.into_iter().map(|c| c.unwrap().buffer).flatten().collect();

        Ok(file)
    }

    ///
    ///
    ///
    pub async fn read_symbolic_link<P>(
        &mut self,
        path: P,
    ) -> Result<FileSystemPath>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        let response = self.client.read_symbolic_link(request).await?;
        let response = response.into_inner();

        let target = FileSystemPath::from(response);
        Ok(target)
    }

    ///
    ///
    ///
    pub async fn rename<P1, P2>(
        &mut self,
        source: P1,
        target: P2,
    ) -> Result<()>
    where
        P1: Into<FileSystemPath>,
        P2: Into<FileSystemPath>,
    {
        let source = source.into();
        let target = target.into();

        let request = x::RenameRequest {
            source: Some(source.proto()),
            target: Some(target.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.rename(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn restore<S1, S2>(
        identifier: S1,
        xenon_endpoint: S2,
    ) -> Result<Self>
    where
        S1: Into<String>,
        S2: Into<String>,
    {
        let mut client = FileSystemServiceClient::connect(xenon_endpoint.into()).await?;
        let filesystem = x::FileSystem { id: identifier.into() };

        // Check if identifier corresponds to an existing and open scheduler.
        let exists_and_open = client.is_open(filesystem.clone()).await?.into_inner().value;
        if !exists_and_open {
            bail!("Identifier '{}' doesn't correspond to an existing and/or open filesystem.");
        }

        let adaptor = client.get_adaptor_name(filesystem.clone()).await?.into_inner().name;
        let identifier = filesystem.id.clone();

        Ok(FileSystem {
            adaptor,
            filesystem,
            identifier,
            client,
        })
    }

    ///
    ///
    ///
    pub async fn set_permissions<P>(
        &mut self,
        path: P,
        permissions: HashSet<FileSystemPermission>,
    ) -> Result<()>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();
        let permissions = permissions.into_iter().map(|p| p.proto().into()).collect();

        let request = x::SetPosixFilePermissionsRequest {
            path: Some(path.proto()),
            permissions,
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.set_posix_file_permissions(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn set_working_directory<P>(
        &mut self,
        path: P,
    ) -> Result<()>
    where
        P: Into<FileSystemPath>,
    {
        let path = path.into();

        let request = x::PathRequest {
            path: Some(path.proto()),
            filesystem: Some(self.filesystem.clone()),
        };

        self.client.set_working_directory(request).await?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn wait_until_done(
        &mut self,
        copy_operation: String,
        timeout: Option<u64>,
    ) -> Result<CopyStatus> {
        let request = x::WaitUntilDoneRequest {
            filesystem: Some(self.filesystem.clone()),
            copy_operation: Some(x::CopyOperation { id: copy_operation }),
            timeout: timeout.unwrap_or_default(),
        };

        let response = self.client.wait_until_done(request).await?;
        let response = response.into_inner();

        Ok(CopyStatus::from(response))
    }

    ///
    ///
    ///
    pub async fn write_to_file<B, P>(
        &mut self,
        buffer: B,
        path: P,
    ) -> Result<()>
    where
        B: Into<Vec<u8>>,
        P: Into<FileSystemPath>,
    {
        let buffer = buffer.into();
        let size = buffer.len() as u64;
        let path = path.into();

        let request = x::WriteToFileRequest {
            path: Some(path.proto()),
            buffer,
            filesystem: Some(self.filesystem.clone()),
            size,
        };

        let request = stream::iter(vec![request]);
        self.client.write_to_file(request).await?;

        Ok(())
    }
}

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct CopyStatus {
    pub bytes_copied: u64,
    pub bytes_to_copy: u64,
    pub copy_operation: String,
    pub done: bool,
    pub error_message: String,
    pub error_type: CopyErrorType,
    pub running: bool,
    pub state: String,
}

impl CopyStatus {
    ///
    ///
    ///
    pub(crate) fn from(status: x::CopyStatus) -> Self {
        let error_type = CopyErrorType::from(status.error_type);

        CopyStatus {
            bytes_copied: status.bytes_copied,
            bytes_to_copy: status.bytes_to_copy,
            copy_operation: status.copy_operation.map(|co| co.id).unwrap_or_default(),
            done: status.done,
            error_message: status.error_message,
            error_type,
            running: status.running,
            state: status.state,
        }
    }
}

///
///
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromPrimitive)]
pub enum CopyErrorType {
    None = 0,
    NotFound = 1,
    Cancelled = 2,
    AlreadyExists = 3,
    NotConnected = 4,
    Xenon = 5,
}

impl CopyErrorType {
    ///
    ///
    ///
    pub(crate) fn from(error_type: i32) -> Self {
        FromPrimitive::from_i32(error_type).expect("Expected a copy status error.")
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
    pub path: Option<FileSystemPath>,
    pub permissions: HashSet<FileSystemPermission>,
    pub size: u64,
}

impl FileSystemAttributes {
    ///
    ///
    ///
    pub(crate) fn from(attributes: x::PathAttributes) -> FileSystemAttributes {
        let path = FileSystemPath::from(attributes.path.unwrap());
        let permissions = attributes
            .permissions
            .into_iter()
            .map(FileSystemPermission::from)
            .collect();

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
            path: Some(path),
            permissions,
            size: attributes.size,
        }
    }
}

///
///
///
#[derive(Clone, Debug, PartialEq)]
pub struct FileSystemPath {
    pub path: PathBuf,
}

impl FileSystemPath {
    ///
    ///
    ///
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        FileSystemPath { path: path.into() }
    }

    ///
    ///
    ///
    pub(crate) fn from(path: x::Path) -> Self {
        FileSystemPath::new(path.path)
    }

    ///
    ///
    ///
    pub(crate) fn proto(&self) -> x::Path {
        let path = self.path.clone().into_os_string().into_string().unwrap();

        x::Path {
            separator: String::from("/"),
            path,
        }
    }
}

impl From<String> for FileSystemPath {
    fn from(path: String) -> Self {
        FileSystemPath::new(path)
    }
}

impl From<&str> for FileSystemPath {
    fn from(path: &str) -> Self {
        FileSystemPath::new(path)
    }
}

impl From<PathBuf> for FileSystemPath {
    fn from(path: PathBuf) -> Self {
        FileSystemPath::new(path)
    }
}

impl From<&FileSystemPath> for FileSystemPath {
    fn from(path: &FileSystemPath) -> Self {
        path.clone()
    }
}

///
///
///
#[derive(Clone, Debug, Eq, Hash, PartialEq, FromPrimitive)]
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
    pub(crate) fn from(permission: i32) -> FileSystemPermission {
        FromPrimitive::from_i32(permission).expect("Expected a POSIX file permission.")
    }

    ///
    ///
    ///
    pub fn proto(self) -> x::PosixFilePermission {
        x::PosixFilePermission::from_i32(self as i32).expect("Expected a POSIX file permission.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xenon as x;

    #[test]
    fn filesystempermission_fromproto_ok() {
        let none = x::PosixFilePermission::None as i32;
        let owner_read = x::PosixFilePermission::OwnerRead as i32;
        let owner_write = x::PosixFilePermission::OwnerWrite as i32;
        let owner_execute = x::PosixFilePermission::OwnerExecute as i32;
        let group_read = x::PosixFilePermission::GroupRead as i32;
        let group_write = x::PosixFilePermission::GroupWrite as i32;
        let group_execute = x::PosixFilePermission::GroupExecute as i32;
        let others_read = x::PosixFilePermission::OthersRead as i32;
        let others_write = x::PosixFilePermission::OthersWrite as i32;
        let others_execute = x::PosixFilePermission::OthersExecute as i32;

        assert_eq!(FileSystemPermission::from(none), FileSystemPermission::None);

        assert_eq!(FileSystemPermission::from(owner_read), FileSystemPermission::OwnerRead);
        assert_eq!(
            FileSystemPermission::from(owner_write),
            FileSystemPermission::OwnerWrite
        );
        assert_eq!(
            FileSystemPermission::from(owner_execute),
            FileSystemPermission::OwnerExecute
        );

        assert_eq!(FileSystemPermission::from(group_read), FileSystemPermission::GroupRead);
        assert_eq!(
            FileSystemPermission::from(group_write),
            FileSystemPermission::GroupWrite
        );
        assert_eq!(
            FileSystemPermission::from(group_execute),
            FileSystemPermission::GroupExecute
        );

        assert_eq!(
            FileSystemPermission::from(others_read),
            FileSystemPermission::OthersRead
        );
        assert_eq!(
            FileSystemPermission::from(others_write),
            FileSystemPermission::OthersWrite
        );
        assert_eq!(
            FileSystemPermission::from(others_execute),
            FileSystemPermission::OthersExecute
        );
    }

    #[test]
    fn filesystempermission_toproto_ok() {
        let none = FileSystemPermission::None;
        let owner_read = FileSystemPermission::OwnerRead;
        let owner_write = FileSystemPermission::OwnerWrite;
        let owner_execute = FileSystemPermission::OwnerExecute;
        let group_read = FileSystemPermission::GroupRead;
        let group_write = FileSystemPermission::GroupWrite;
        let group_execute = FileSystemPermission::GroupExecute;
        let others_read = FileSystemPermission::OthersRead;
        let others_write = FileSystemPermission::OthersWrite;
        let others_execute = FileSystemPermission::OthersExecute;

        assert_eq!(none.proto(), x::PosixFilePermission::None);

        assert_eq!(owner_read.proto(), x::PosixFilePermission::OwnerRead);
        assert_eq!(owner_write.proto(), x::PosixFilePermission::OwnerWrite);
        assert_eq!(owner_execute.proto(), x::PosixFilePermission::OwnerExecute);

        assert_eq!(group_read.proto(), x::PosixFilePermission::GroupRead);
        assert_eq!(group_write.proto(), x::PosixFilePermission::GroupWrite);
        assert_eq!(group_execute.proto(), x::PosixFilePermission::GroupExecute);

        assert_eq!(others_read.proto(), x::PosixFilePermission::OthersRead);
        assert_eq!(others_write.proto(), x::PosixFilePermission::OthersWrite);
        assert_eq!(others_execute.proto(), x::PosixFilePermission::OthersExecute);
    }
}
