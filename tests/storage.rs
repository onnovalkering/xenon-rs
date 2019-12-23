mod common;
use rand::random;
use xenon_rs::storage::FileSystemPath;

#[test]
fn appendtofile_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    filesystem.create_file(path.clone()).unwrap();

    let buffer = String::from("Hello, world!").as_bytes().to_vec();
    let result = filesystem.append_to_file(buffer, path);

    assert!(result.is_ok())
}

#[test]
fn appendtofile_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let buffer = String::new().as_bytes().to_vec();
    let result = filesystem.append_to_file(buffer, path);

    assert!(result.is_err());
}

#[test]
fn create_default_ok() {
    let filesystem = common::create_sftp_filesystem();

    assert!(filesystem.is_ok());
}

#[test]
fn createdirectories_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("filesystem-test-fixture/links"));
    let result = filesystem.create_directories(path);

    assert!(result.is_err());
}

#[test]
fn createdirectories_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(format!("directory_{}/sub/sub", random::<u16>()));
    let result = filesystem.create_directories(path);

    assert!(result.is_ok());
}

#[test]
fn createdirectory_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("filesystem-test-fixture"));
    let result = filesystem.create_directory(path);

    assert!(result.is_err());
}

#[test]
fn createdirectory_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(format!("directory_{}", random::<u16>()));
    let result = filesystem.create_directory(path);

    assert!(result.is_ok());
}

#[test]
fn createfile_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("test-slurm.job"));
    let result = filesystem.create_file(path);

    assert!(result.is_err());
}

#[test]
fn createfile_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let result = filesystem.create_file(path);

    assert!(result.is_ok());
}

#[test]
fn createsymboliclink_existing_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(String::from("test-slurm.job"));
    let target = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.create_symbolic_link(link.clone(), target.clone());

    assert!(result.is_err());
}

#[test]
fn createsymboliclink_nonexisting_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let target = FileSystemPath::new(String::from("test-slurm.job"));
    filesystem.create_symbolic_link(link.clone(), target.clone()).unwrap();

    let result = filesystem.read_symbolic_link(link);

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));
}

#[test]
fn createsymboliclink_nonexistingtarget_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let target = FileSystemPath::new(String::from("nonexisting.txt"));
    filesystem.create_symbolic_link(link.clone(), target.clone()).unwrap();

    let result = filesystem.read_symbolic_link(link);

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));
}

#[test]
fn delete_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.delete(path, false);

    assert!(result.is_err());
}

#[test]
fn delete_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.delete(path, false);

    assert!(result.is_err());
}

#[test]
fn exists_existing_true() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("test-slurm.job"));
    let result = filesystem.exists(path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), true);
}

#[test]
fn exists_nonexisting_false() {
    let filesystem = common::create_sftp_filesystem().unwrap();
 
    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.exists(path);

    assert!(result.is_ok());
    assert_eq!(result.unwrap(), false);
}

#[test]
fn readfromfile_existing_buffer() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("test-slurm.job"));
    let result = filesystem.read_from_file(path);

    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_some());
    let buffer = result.unwrap();
    assert_eq!(buffer, common::get_slurmjob_file());
}

#[test]
fn readfromfile_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.read_from_file(path);

    assert!(result.is_err());
}

#[test]
fn readsymboliclink_existing_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    let target = FileSystemPath::new(format!("file_{}.txt", random::<u16>()));
    filesystem.create_symbolic_link(link.clone(), target.clone()).unwrap();

    let result = filesystem.read_symbolic_link(link);

    assert!(result.is_ok());
    assert!(result.unwrap().path.ends_with(&target.path));
}

#[test]
fn readsymboliclink_nonexisting_err() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.read_symbolic_link(link);

    assert!(result.is_err());
}

#[test]
fn readsymboliclink_nonsymboliclink_ok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let link = FileSystemPath::new(String::from("test-slurm.job"));
    let result = filesystem.read_symbolic_link(link);

    assert!(result.is_err());
}