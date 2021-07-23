use anyhow::Result;
use rand::random;
use std::collections::HashMap;
use std::collections::HashSet;
use xenon::credentials::Credential;
use xenon::storage::FileSystem;
use xenon::storage::{FileSystemPath, FileSystemPermission};


pub async fn create_sftp_filesystem() -> Result<FileSystem> {
    let credential = Credential::new_password(String::from("xenon"), String::from("javagat"));
    create_sftp_filesystem_inner(credential).await
}

pub async fn create_sftp_filesystem_inner(credential: Credential) -> Result<FileSystem> {
    let mut properties = HashMap::new();
    properties.insert(
        String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking"),
        String::from("false"),
    );

    let filesystem = FileSystem::create(
        String::from("sftp"),
        String::from("slurm:22"),
        credential,
        "http://localhost:50051",
        Some(properties),
    )
    .await?;

    Ok(filesystem)
}

///
///
///
pub fn get_slurmjob_file() -> Vec<u8> {
    let slurmjob = concat!(
        "#!/bin/bash\n",
        "#SBATCH --job-name test-slurm\n",
        "#SBATCH --output test-slurm.out\n",
        "#SBATCH --error test-slurm.err\n",
        "#SBATCH --time 0:30:00\n",
        "#SBATCH --partition mypartition\n",
        "#SBATCH --ntasks 1\n",
        "\n",
        "date\n",
        "hostname\n",
        "sleep 15\n",
        "date\n"
    );

    slurmjob.as_bytes().to_vec()
}

#[tokio::test]
async fn appendtofile_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    filesystem.create_file(&path).await?;

    let buffer = String::from("Hello, world!").as_bytes().to_vec();
    let result = filesystem.append_to_file(buffer, &path).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn appendtofile_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let buffer = String::new().as_bytes().to_vec();
    let result = filesystem.append_to_file(buffer, &path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn copy_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let source = FileSystemPath::new("test-slurm.job");
    let destination = FileSystemPath::new(format!("copy_{}.txt", random::<u16>()));
    let result = filesystem.copy(&source, &destination, None, false, 5000).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn copy_existingremotefs_ok() -> Result<()> {
    let mut filesystem1 = create_sftp_filesystem().await?;
    let filesystem2 = create_sftp_filesystem().await?;

    let source = FileSystemPath::new("test-slurm.job");
    let destination = FileSystemPath::new(format!("copy_{}.txt", random::<u16>()));
    let result = filesystem1
        .copy(&source, &destination, Some(filesystem2), false, 5000)
        .await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn create_passwordcredential_ok() -> Result<()> {
    let filesystem = create_sftp_filesystem().await;

    assert!(filesystem.is_ok());

    Ok(())
}

#[tokio::test]
async fn create_certificatecredential_ok() -> Result<()> {
    let credential = Credential::new_certificate("/unsafe_ssh_key", "xenon", "");
    let scheduler = create_sftp_filesystem_inner(credential).await;
    assert!(scheduler.is_ok());

    Ok(())
}

#[tokio::test]
async fn createdirectories_existing_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("filesystem-test-fixture/links");
    let result = filesystem.create_directories(&path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn createdirectories_nonexisting_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new(format!("directory_{}/sub/sub", random::<u16>()));
    let result = filesystem.create_directories(&path).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn createdirectory_existing_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("filesystem-test-fixture");
    let result = filesystem.create_directory(&path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn createdirectory_nonexisting_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new(format!("directory_{}", random::<u16>()));
    let result = filesystem.create_directory(&path).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn createfile_existing_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("test-slurm.job");
    let result = filesystem.create_file(&path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn createfile_nonexisting_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let result = filesystem.create_file(&path).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn createsymboliclink_existing_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let link = FileSystemPath::new("test-slurm.job");
    let target = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.create_symbolic_link(&link, &target).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn createsymboliclink_nonexisting_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let link = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let target = FileSystemPath::new("test-slurm.job");
    filesystem.create_symbolic_link(&link, &target).await?;

    let result = filesystem.read_symbolic_link(&link).await;

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));

    Ok(())
}

#[tokio::test]
async fn createsymboliclink_nonexistingtarget_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let link = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let target = FileSystemPath::new("nonexisting.txt");
    filesystem.create_symbolic_link(&link, &target).await?;

    let result = filesystem.read_symbolic_link(&link).await;

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));

    Ok(())
}

#[tokio::test]
async fn delete_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.delete(&path, false).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn delete_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.delete(&path, false).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn exists_existing_true() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("test-slurm.job");
    let result = filesystem.exists(&path).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    Ok(())
}

#[tokio::test]
async fn exists_nonexisting_false() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.exists(&path).await;

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    Ok(())
}

#[tokio::test]
async fn getattributes_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("test-slurm.job");
    let result = filesystem.exists(&path).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn getattributes_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.get_attributes(&path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn getfscredential_default_password() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let result = filesystem.get_fs_credential().await;

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

    Ok(())
}

#[tokio::test]
async fn getfslocation_default_location() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let result = filesystem.get_fs_location().await;
    assert!(result.is_ok());
    let location = result.unwrap();

    assert_eq!(location, String::from("slurm:22"));

    Ok(())
}

#[tokio::test]
async fn getfsproperties_default_properties() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let result = filesystem.get_fs_properties().await;
    assert!(result.is_ok());
    let properties = result.unwrap();
    assert!(properties.contains_key(&String::from("xenon.adaptors.filesystems.sftp.strictHostKeyChecking")));

    Ok(())
}

#[tokio::test]
async fn getfsseparator_default_separator() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let result = filesystem.get_fs_separator().await;
    assert!(result.is_ok());
    let separator = result.unwrap();

    assert_eq!(separator, String::from("/"));

    Ok(())
}

#[tokio::test]
async fn getworkingdirectory_default_path() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let result = filesystem.get_working_directory().await;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn isopen_open_true() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let result = filesystem.is_open().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);

    Ok(())
}

#[tokio::test]
async fn isopen_closed_false() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;
    filesystem.close().await?;

    let result = filesystem.is_open().await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);

    Ok(())
}

#[tokio::test]
async fn list_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("/home/xenon");
    let result = filesystem.list(&path, false).await;

    assert!(result.is_ok());
    let files = result.unwrap();
    assert!(files.len() > 0);

    Ok(())
}

#[tokio::test]
async fn list_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("/nonexisting");
    let result = filesystem.list(&path, false).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn readfromfile_existing_buffer() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("test-slurm.job");
    let result = filesystem.read_from_file(&path).await;

    assert!(result.is_ok());
    let buffer = result.unwrap();
    assert_eq!(buffer, get_slurmjob_file());

    Ok(())
}

#[tokio::test]
async fn readfromfile_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.read_from_file(&path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn readsymboliclink_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let link = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let target = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    filesystem.create_symbolic_link(&link, &target).await?;

    let result = filesystem.read_symbolic_link(&link).await;

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));

    Ok(())
}

#[tokio::test]
async fn readsymboliclink_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let link = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.read_symbolic_link(&link).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn readsymboliclink_nonsymboliclink_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let link = FileSystemPath::new("test-slurm.job");
    let result = filesystem.read_symbolic_link(&link).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn rename_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let source = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    filesystem.create_file(&source).await?;

    let destination = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let result = filesystem.rename(&source, &destination).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn rename_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let source = FileSystemPath::new("nonexisting.txt");
    let destination = FileSystemPath::new("nonexisting.txt");
    let result = filesystem.rename(&source, &destination).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn setpermissions_existing_ok() -> Result<()> {
    use FileSystemPermission::*;
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("test-slurm.job");
    let mut permissions = HashSet::<FileSystemPermission>::new();
    permissions.extend(vec![OwnerRead, OwnerWrite, OwnerExecute]);

    let result = filesystem.set_permissions(&path, permissions.clone()).await;

    assert!(result.is_ok());
    let attributes = filesystem.get_attributes(&path).await?;
    assert_eq!(permissions, attributes.permissions);

    Ok(())
}

#[tokio::test]
async fn setpermissions_nonexisting_err() -> Result<()> {
    use FileSystemPermission::*;
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new("nonexisting.txt");
    let mut permissions = HashSet::<FileSystemPermission>::new();
    permissions.extend(vec![OwnerRead, OwnerWrite, OwnerExecute]);

    let result = filesystem.set_permissions(&path, permissions).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn setworkingdirectory_existing_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let directory = FileSystemPath::new("/home/xenon/filesystem-test-fixture");
    let result = filesystem.set_working_directory(&directory).await;

    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn setworkingdirectory_nonexisting_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let directory = FileSystemPath::new("/nonexisting");
    let result = filesystem.set_working_directory(&directory).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn writetofile_existing_err() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    filesystem.create_file(&path).await?;

    let buffer = String::from("Hello, world!").as_bytes().to_vec();
    let result = filesystem.write_to_file(buffer, &path).await;

    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn writetofile_nonexisting_ok() -> Result<()> {
    let mut filesystem = create_sftp_filesystem().await?;

    let path = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let buffer = String::new().as_bytes().to_vec();
    let result = filesystem.write_to_file(buffer, &path).await;

    assert!(result.is_ok());

    Ok(())
}
