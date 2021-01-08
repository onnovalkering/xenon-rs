use anyhow::Result;
use crate::credentials::Credential;
use crate::xenon as x;
use crate::xenon_grpc::FileSystemServiceClient;
use futures::{StreamExt, SinkExt};
use grpcio::{Channel, WriteFlags};
use std::collections::HashSet;
use std::path::PathBuf;

type Map<T> = std::collections::HashMap<String, T>;

///
///
///
pub struct FileSystem {
    pub adaptor: String,
    client: FileSystemServiceClient,
    open: bool,
    pub(crate) filesystem: x::FileSystem,
    pub identifier: String,
}

impl FileSystem {
    ///
    ///
    ///
    pub async fn append_to_file(
        &self,
        buffer: Vec<u8>,
        path: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::AppendToFileRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_buffer(buffer);

        let (mut sink, receiver) = self.client.append_to_file()?;

        sink.send((request, WriteFlags::default())).await?;

        sink.close().await?;
        receiver.await?;

        Ok(())
    }

    ///
    ///
    ///
    pub fn close(&mut self) -> Result<()> {
        if self.open {
            debug!("Closing filesystem: {}", self.identifier);

            self.client.close(&self.filesystem)?;
            self.open = false;
        }

        Ok(())
    }

    ///
    ///
    ///
    pub fn copy(
        &self,
        source: &FileSystemPath,
        destination: &FileSystemPath,
        destination_filesystem: Option<FileSystem>,
        recursive: bool,
        timeout: u64,
    ) -> Result<()> {
        let mut request = x::CopyRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_source(source.proto());
        request.set_destination(destination.proto());
        request.set_recursive(recursive);

        let mut _remote_fs;
        if let Some(remote) = destination_filesystem {
            request.set_destination_filesystem(remote.filesystem.clone());
            _remote_fs = remote;
        } else {
            request.set_destination_filesystem(self.filesystem.clone());
        };

        let operation = self.client.copy(&request)?;

        let mut request = x::WaitUntilDoneRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_copy_operation(operation);
        request.set_timeout(timeout);

        self.client.wait_until_done(&request)?;

        Ok(())
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
    ) -> Result<FileSystem> {
        let client = FileSystemServiceClient::new(channel);

        // Construct create request message.
        let mut request = x::CreateFileSystemRequest::new();
        request.set_adaptor(adaptor.clone());
        request.set_location(location);
        request.set_properties(properties);
        match credential {
            Credential::Password(password) => request.set_password_credential(password.proto()),
            Credential::Certificate(certificate) => request.set_certificate_credential(certificate.proto()),
        }

        let filesystem = client.create(&request)?;
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
    pub fn create_directories(
        &self,
        path: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::PathRequest::new();
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
        path: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::PathRequest::new();
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
        path: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::PathRequest::new();
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
        link: &FileSystemPath,
        target: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::CreateSymbolicLinkRequest::new();
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
        path: &FileSystemPath,
        recursive: bool,
    ) -> Result<()> {
        let mut request = x::DeleteRequest::new();
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
        path: &FileSystemPath,
    ) -> Result<bool> {
        let mut request = x::PathRequest::new();
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
        path: &FileSystemPath,
    ) -> Result<FileSystemAttributes> {
        let mut request = x::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let attributes = self.client.get_attributes(&request)?;
        let attributes = FileSystemAttributes::from(attributes);

        Ok(attributes)
    }

    ///
    ///
    ///
    pub fn get_fs_credential(&self) -> Result<Option<Credential>> {
        let response = self.client.get_credential(&self.filesystem)?;
        if let Some(one_of_credential) = response.credential {
            use x::GetCredentialResponse_oneof_credential::*;

            match one_of_credential {
                certificate_credential(credential) => Ok(Some(Credential::new_certificate(
                    credential.certfile,
                    credential.username,
                    credential.passphrase,
                ))),
                password_credential(credential) => {
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
    pub fn get_fs_location(&self) -> Result<String> {
        let response = self.client.get_location(&self.filesystem)?;

        Ok(response.location)
    }

    ///
    ///
    ///
    pub fn get_fs_separator(&self) -> Result<String> {
        let response = self.client.get_path_separator(&self.filesystem)?;

        Ok(response.separator)
    }

    ///
    ///
    ///
    pub fn get_fs_properties(&self) -> Result<Map<String>> {
        let response = self.client.get_properties(&self.filesystem)?;

        Ok(response.properties)
    }

    ///
    ///
    ///
    pub fn get_working_directory(&self) -> Result<FileSystemPath> {
        let directory = self.client.get_working_directory(&self.filesystem)?;
        let directory = FileSystemPath::from(directory);


        Ok(directory)
    }

    ///
    ///
    ///
    pub fn is_open(&self) -> Result<bool> {
        if !self.open {
            return Ok(false);
        }

        let is = self.client.is_open(&self.filesystem)?;

        Ok(is.value)
    }

    ///
    ///
    ///
    pub async fn list(
        &self,
        path: &FileSystemPath,
        recursive: bool,
    ) -> Result<Vec<FileSystemAttributes>> {
        let mut request = x::ListRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_dir(path.proto());
        request.set_recursive(recursive);

        let reciever = self.client.list(&request)?;
        let files = reciever.collect::<Vec<Result<x::PathAttributes, grpcio::Error>>>().await;

        let mut files = files.into_iter();

        // In case the path does not exist,
        let files = if files.len() == 1 {
            let file = files.next().unwrap();
            match file {
                Ok(f) => vec![FileSystemAttributes::from(f)],
                Err(_) => {
                    bail!("Couldn't list files in path.")
                }
            }
        } else {
            files
                .into_iter()
                .filter(|f| f.is_ok())
                .map(|f| {
                    FileSystemAttributes::from(f.unwrap())
                })
                .collect()
        };

        Ok(files)
    }

    ///
    ///
    ///
    pub fn rename(
        &self,
        source: &FileSystemPath,
        target: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::RenameRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_source(source.proto());
        request.set_target(target.proto());

        self.client.rename(&request)?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn read_from_file(
        &self,
        path: &FileSystemPath,
    ) -> Result<Option<Vec<u8>>> {
        let mut request = x::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let mut receiver = self.client.read_from_file(&request)?;

        if let Some(Ok(file)) = receiver.next().await {
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
        path: &FileSystemPath,
    ) -> Result<FileSystemPath> {
        let mut request = x::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        let target = self.client.read_symbolic_link(&request)?;
        let target = FileSystemPath::from(target);

        Ok(target)
    }

    ///
    ///
    ///
    pub fn set_permissions(
        &self,
        path: &FileSystemPath,
        permissions: HashSet<FileSystemPermission>,
    ) -> Result<()> {
        let mut request = x::SetPosixFilePermissionsRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_permissions(permissions.iter().map(|p| p.proto()).collect());

        self.client.set_posix_file_permissions(&request)?;

        Ok(())
    }

    ///
    ///
    ///
    pub fn set_working_directory(
        &self,
        path: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::PathRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());

        self.client.set_working_directory(&request)?;

        Ok(())
    }

    ///
    ///
    ///
    pub async fn write_to_file(
        &self,
        buffer: Vec<u8>,
        path: &FileSystemPath,
    ) -> Result<()> {
        let mut request = x::WriteToFileRequest::new();
        request.set_filesystem(self.filesystem.clone());
        request.set_path(path.proto());
        request.set_buffer(buffer);

        let (mut sink, receiver) = self.client.write_to_file()?;
        sink.send((request, WriteFlags::default())).await?;
        sink.close().await?;

        receiver.await?;

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
            .iter()
            .map(|p| FileSystemPermission::from(*p))
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
    pub fn new(path: PathBuf) -> Self {
        FileSystemPath { path }
    }

    ///
    ///
    ///
    pub(crate) fn from(path: x::Path) -> Self {
        let pathbuf = PathBuf::from(path.path);
        FileSystemPath::new(pathbuf)
    }

    ///
    ///
    ///
    pub(crate) fn proto(&self) -> x::Path {
        let mut path = x::Path::new();
        path.set_path(self.path.clone().into_os_string().into_string().unwrap());

        path
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
        use FileSystemPermission::*;

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
    pub fn proto(&self) -> x::PosixFilePermission {
        use x::PosixFilePermission::*;
        use FileSystemPermission::*;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xenon as x;


    #[test]
    fn filesystempermission_fromproto_ok() {
        let none = x::PosixFilePermission::NONE;
        let owner_read = x::PosixFilePermission::OWNER_READ;
        let owner_write = x::PosixFilePermission::OWNER_WRITE;
        let owner_execute = x::PosixFilePermission::OWNER_EXECUTE;
        let group_read = x::PosixFilePermission::GROUP_READ;
        let group_write = x::PosixFilePermission::GROUP_WRITE;
        let group_execute = x::PosixFilePermission::GROUP_EXECUTE;
        let others_read = x::PosixFilePermission::OTHERS_READ;
        let others_write = x::PosixFilePermission::OTHERS_WRITE;
        let others_execute = x::PosixFilePermission::OTHERS_EXECUTE;

        assert_eq!(FileSystemPermission::from(none), FileSystemPermission::None);

        assert_eq!(FileSystemPermission::from(owner_read), FileSystemPermission::OwnerRead);
        assert_eq!(FileSystemPermission::from(owner_write), FileSystemPermission::OwnerWrite);
        assert_eq!(FileSystemPermission::from(owner_execute), FileSystemPermission::OwnerExecute);

        assert_eq!(FileSystemPermission::from(group_read), FileSystemPermission::GroupRead);
        assert_eq!(FileSystemPermission::from(group_write), FileSystemPermission::GroupWrite);
        assert_eq!(FileSystemPermission::from(group_execute), FileSystemPermission::GroupExecute);

        assert_eq!(FileSystemPermission::from(others_read), FileSystemPermission::OthersRead);
        assert_eq!(FileSystemPermission::from(others_write), FileSystemPermission::OthersWrite);
        assert_eq!(FileSystemPermission::from(others_execute), FileSystemPermission::OthersExecute);
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

        assert_eq!(none.proto(), x::PosixFilePermission::NONE);

        assert_eq!(owner_read.proto(), x::PosixFilePermission::OWNER_READ);
        assert_eq!(owner_write.proto(), x::PosixFilePermission::OWNER_WRITE);
        assert_eq!(owner_execute.proto(), x::PosixFilePermission::OWNER_EXECUTE);

        assert_eq!(group_read.proto(), x::PosixFilePermission::GROUP_READ);
        assert_eq!(group_write.proto(), x::PosixFilePermission::GROUP_WRITE);
        assert_eq!(group_execute.proto(), x::PosixFilePermission::GROUP_EXECUTE);

        assert_eq!(others_read.proto(), x::PosixFilePermission::OTHERS_READ);
        assert_eq!(others_write.proto(), x::PosixFilePermission::OTHERS_WRITE);
        assert_eq!(others_execute.proto(), x::PosixFilePermission::OTHERS_EXECUTE);
    }
}
