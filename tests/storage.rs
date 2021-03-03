mod common;
use rand::random;
use std::collections::HashSet;
use xenon::credentials::Credential;
use xenon::storage::{FileSystemPath, FileSystemPermission};
use std::path::PathBuf;

#[tokio::test]
async fn appendtofile_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    filesystem.create_file(&path).unwrap();

    let buffer = String::from("Hello, world!").as_bytes().to_vec();
    let result = filesystem.append_to_file(buffer, &path).await;

    assert!(result.is_ok())
}

#[tokio::test]
async fn appendtofile_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let buffer = String::new().as_bytes().to_vec();
    let result = filesystem.append_to_file(buffer, &path).await;

    assert!(result.is_err());
}

#[test]
fn copy_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let source = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let destination = FileSystemPath::new(PathBuf::from(format!("copy_{}.txt", random::<u16>())));
    let result = filesystem.copy(&source, &destination, None, false, 5000);

    assert!(result.is_ok());
}

#[test]
fn copy_existingremotefs_ok() {
    let filesystem1 = common::create_sftp_filesystem().unwrap();
    let filesystem2 = common::create_sftp_filesystem().unwrap();

    let source = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let destination = FileSystemPath::new(PathBuf::from(format!("copy_{}.txt", random::<u16>())));
    let result = filesystem1.copy(&source, &destination, Some(filesystem2), false, 5000);

    assert!(result.is_ok());
}

#[test]
fn create_default_ok() {
    let filesystem = common::create_sftp_filesystem();

    assert!(filesystem.is_ok());
}

#[test]
fn createdirectories_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("filesystem-test-fixture/links"));
    let result = filesystem.create_directories(&path);

    assert!(result.is_err());
}

#[test]
fn createdirectories_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from(format!("directory_{}/sub/sub", random::<u16>())));
    let result = filesystem.create_directories(&path);

    assert!(result.is_ok());
}

#[test]
fn createdirectory_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("filesystem-test-fixture"));
    let result = filesystem.create_directory(&path);

    assert!(result.is_err());
}

#[test]
fn createdirectory_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from(format!("directory_{}", random::<u16>())));
    let result = filesystem.create_directory(&path);

    assert!(result.is_ok());
}

#[test]
fn createfile_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let result = filesystem.create_file(&path);

    assert!(result.is_err());
}

#[test]
fn createfile_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    let result = filesystem.create_file(&path);

    assert!(result.is_ok());
}

#[test]
fn createsymboliclink_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let target = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.create_symbolic_link(&link, &target);

    assert!(result.is_err());
}

#[test]
fn createsymboliclink_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    let target = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    filesystem.create_symbolic_link(&link, &target).unwrap();

    let result = filesystem.read_symbolic_link(&link);

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));
}

#[test]
fn createsymboliclink_nonexistingtarget_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    let target = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    filesystem.create_symbolic_link(&link, &target).unwrap();

    let result = filesystem.read_symbolic_link(&link);

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));
}

#[test]
fn delete_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.delete(&path, false);

    assert!(result.is_err());
}

#[test]
fn delete_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.delete(&path, false);

    assert!(result.is_err());
}

#[test]
fn exists_existing_true() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let result = filesystem.exists(&path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn exists_nonexisting_false() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.exists(&path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
fn getattributes_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let result = filesystem.exists(&path);

    assert!(result.is_ok());
}

#[test]
fn getattributes_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.get_attributes(&path);

    assert!(result.is_err());
}

#[test]
fn getfscredential_default_password() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let result = filesystem.get_fs_credential();

    assert!(result.is_ok());
    let credential = result.unwrap();
    assert!(credential.is_some());

    let credential = credential.unwrap();
    if let Credential::Password(credential) = credential {
        assert_eq!(credential.username, String::from("xenon"));
        assert_eq!(credential.password, String::from("javagat"));
    } else {
        unreachable!();
    }
}

#[test]
fn getfslocation_default_location() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let result = filesystem.get_fs_location();
    assert!(result.is_ok());
    let location = result.unwrap();

    assert_eq!(location, String::from("slurm:22"));
}

#[test]
fn getfsproperties_default_properties() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let result = filesystem.get_fs_properties();
    assert!(result.is_ok());
    let properties = result.unwrap();
    assert!(properties.contains_key(&String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking")))
}

#[test]
fn getfsseparator_default_separator() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let result = filesystem.get_fs_separator();
    assert!(result.is_ok());
    let separator = result.unwrap();

    assert_eq!(separator, String::from("/"));
}

#[test]
fn getworkingdirectory_default_path() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let result = filesystem.get_working_directory();
    assert!(result.is_ok());
}

#[test]
fn isopen_open_true() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let result = filesystem.is_open();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn isopen_closed_false() {
    let mut filesystem = common::create_sftp_filesystem().unwrap();
    filesystem.close().unwrap();

    let result = filesystem.is_open();
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[tokio::test]
async fn list_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("/home/xenon"));
    let result = filesystem.list(&path, false).await;

    assert!(result.is_ok());
    let files = result.unwrap();
    assert!(files.len() > 0);
}

#[tokio::test]
async fn list_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("/nonexisting"));
    let result = filesystem.list(&path, false).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn readfromfile_existing_buffer() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let result = filesystem.read_from_file(&path).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let buffer = result.unwrap();
    assert_eq!(buffer, common::get_slurmjob_file());
}

#[tokio::test]
async fn readfromfile_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.read_from_file(&path).await;

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_none());
}

#[test]
fn readsymboliclink_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    let target = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    filesystem.create_symbolic_link(&link, &target).unwrap();

    let result = filesystem.read_symbolic_link(&link);

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));
}

#[test]
fn readsymboliclink_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.read_symbolic_link(&link);

    assert!(result.is_err());
}

#[test]
fn readsymboliclink_nonsymboliclink_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let result = filesystem.read_symbolic_link(&link);

    assert!(result.is_err());
}

#[test]
fn rename_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let source = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    filesystem.create_file(&source).unwrap();

    let destination = FileSystemPath::new(PathBuf::from(format!("file_{}.txt", random::<u16>())));
    let result = filesystem.rename(&source, &destination);

    assert!(result.is_ok());
}

#[test]
fn rename_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let source = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let destination = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let result = filesystem.rename(&source, &destination);

    assert!(result.is_err());
}

#[test]
fn setpermissions_existing_ok() {
    use FileSystemPermission::*;
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("test-slurm.job"));
    let mut permissions = HashSet::<FileSystemPermission>::new();
    permissions.extend(vec![OwnerRead, OwnerWrite, OwnerExecute]);

    let result = filesystem.set_permissions(&path, permissions.clone());

    assert!(result.is_ok());
    let attributes = filesystem.get_attributes(&path).unwrap();
    assert_eq!(permissions, attributes.permissions);
}

#[test]
fn setpermissions_nonexisting_err() {
    use FileSystemPermission::*;
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(PathBuf::from("nonexisting.txt"));
    let mut permissions = HashSet::<FileSystemPermission>::new();
    permissions.extend(vec![OwnerRead, OwnerWrite, OwnerExecute]);

    let result = filesystem.set_permissions(&path, permissions);

    assert!(result.is_err());
}

#[test]
fn setworkingdirectory_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let directory = FileSystemPath::new(PathBuf::from("/home/xenon/filesystem-test-fixture"));
    let result = filesystem.set_working_directory(&directory);

    assert!(result.is_ok());
}

#[test]
fn setworkingdirectory_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let directory = FileSystemPath::new(PathBuf::from("/nonexisting"));
    let result = filesystem.set_working_directory(&directory);

    assert!(result.is_err());
}
