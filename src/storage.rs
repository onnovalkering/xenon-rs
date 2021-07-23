use crate::credentials::Credential;
use crate::xenon::{self as x, file_system_service_client::FileSystemServiceClient};
use anyhow::Result;
use futures::StreamExt;
use futures_util::stream;
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
    open: bool,
    pub(crate) filesystem: x::FileSystem,
    pub identifier: String,
}

impl FileSystem {
    ///
    ///
    ///
    pub async fn append_to_file<B: Into<Vec<u8>>>(
        &mut self,
        buffer: B,
        path: &FileSystemPath,
    ) -> Result<()> {
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
        if self.open {
            debug!("Closing filesystem: {}", self.identifier);

            self.client.close(self.filesystem.clone()).await?;
            self.open = false;
        }

        Ok(())
    }

    ///
    ///
    ///
    pub async fn copy(
        &mut self,
        source: &FileSystemPath,
        destination: &FileSystemPath,
        destination_filesystem: Option<FileSystem>,
        recursive: bool,
        timeout: u64,
    ) -> Result<()> {
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

        let request = x::WaitUntilDoneRequest {
            copy_operation: Some(response),
            filesystem: Some(self.filesystem.clone()),
            timeout,
        };

        self.client.wait_until_done(request).await?;

        Ok(())
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
    ) -> Result<FileSystem>
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
            open: true,
            identifier,
            client,
        })
    }

    ///
    ///
    ///
    pub async fn create_directories(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<()> {
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
    pub async fn create_directory(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<()> {
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
    pub async fn create_file(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<()> {
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
    pub async fn create_symbolic_link(
        &mut self,
        link: &FileSystemPath,
        target: &FileSystemPath,
    ) -> Result<()> {
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
    pub async fn delete(
        &mut self,
        path: &FileSystemPath,
        recursive: bool,
    ) -> Result<()> {
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
    pub async fn exists(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<bool> {
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
    pub async fn get_attributes(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<FileSystemAttributes> {
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
    pub async fn get_fs_credential(&mut self) -> Result<Option<Credential>> {
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
    pub async fn get_fs_location(&mut self) -> Result<String> {
        let response = self.client.get_location(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.location)
    }

    ///
    ///
    ///
    pub async fn get_fs_separator(&mut self) -> Result<String> {
        let response = self.client.get_path_separator(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.separator)
    }

    ///
    ///
    ///
    pub async fn get_fs_properties(&mut self) -> Result<HashMap<String, String>> {
        let response = self.client.get_properties(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.properties)
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
    pub async fn is_open(&mut self) -> Result<bool> {
        if !self.open {
            return Ok(false);
        }

        let response = self.client.is_open(self.filesystem.clone()).await?;
        let response = response.into_inner();

        Ok(response.value)
    }

    ///
    ///
    ///
    pub async fn list(
        &mut self,
        path: &FileSystemPath,
        recursive: bool,
    ) -> Result<Vec<FileSystemAttributes>> {
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
    pub async fn rename(
        &mut self,
        source: &FileSystemPath,
        target: &FileSystemPath,
    ) -> Result<()> {
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
    pub async fn read_from_file(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<Vec<u8>> {
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
    pub async fn read_symbolic_link(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<FileSystemPath> {
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
    pub async fn set_permissions(
        &mut self,
        path: &FileSystemPath,
        permissions: HashSet<FileSystemPermission>,
    ) -> Result<()> {
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
    pub async fn set_working_directory(
        &mut self,
        path: &FileSystemPath,
    ) -> Result<()> {
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
    pub async fn write_to_file<B: Into<Vec<u8>>>(
        &mut self,
        buffer: B,
        path: &FileSystemPath,
    ) -> Result<()> {
        let buffer = buffer.into();
        let size = buffer.len() as u64;

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
            .map(|p| x::PosixFilePermission::from_i32(p).unwrap_or_default())
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

///
///
///
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    pub(crate) fn from(permission: x::PosixFilePermission) -> FileSystemPermission {
        use x::PosixFilePermission::*;

        match permission {
            None => FileSystemPermission::None,
            OwnerRead => FileSystemPermission::OwnerRead,
            OwnerWrite => FileSystemPermission::OwnerWrite,
            OwnerExecute => FileSystemPermission::OwnerExecute,
            GroupRead => FileSystemPermission::GroupRead,
            GroupWrite => FileSystemPermission::GroupWrite,
            GroupExecute => FileSystemPermission::GroupExecute,
            OthersRead => FileSystemPermission::OthersRead,
            OthersWrite => FileSystemPermission::OthersWrite,
            OthersExecute => FileSystemPermission::OthersExecute,
        }
    }

    ///
    ///
    ///
    pub fn proto(&self) -> x::PosixFilePermission {
        use x::PosixFilePermission::*;

        match self {
            FileSystemPermission::None => None,
            FileSystemPermission::OwnerRead => OwnerRead,
            FileSystemPermission::OwnerWrite => OwnerWrite,
            FileSystemPermission::OwnerExecute => OwnerExecute,
            FileSystemPermission::GroupRead => GroupRead,
            FileSystemPermission::GroupWrite => GroupWrite,
            FileSystemPermission::GroupExecute => GroupExecute,
            FileSystemPermission::OthersRead => OthersRead,
            FileSystemPermission::OthersWrite => OthersWrite,
            FileSystemPermission::OthersExecute => OthersExecute,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xenon as x;

    #[test]
    fn filesystempermission_fromproto_ok() {
        let none = x::PosixFilePermission::None;
        let owner_read = x::PosixFilePermission::OwnerRead;
        let owner_write = x::PosixFilePermission::OwnerWrite;
        let owner_execute = x::PosixFilePermission::OwnerExecute;
        let group_read = x::PosixFilePermission::GroupRead;
        let group_write = x::PosixFilePermission::GroupWrite;
        let group_execute = x::PosixFilePermission::GroupExecute;
        let others_read = x::PosixFilePermission::OthersRead;
        let others_write = x::PosixFilePermission::OthersWrite;
        let others_execute = x::PosixFilePermission::OthersExecute;

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
