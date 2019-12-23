mod common;
use rand::random;
use xenon_rs::storage::FileSystemPath;

#[test]
fn create_default_isok() {
    let filesystem = common::create_sftp_filesystem();

    assert!(filesystem.is_ok());
}

#[test]
fn createfile_existing_iserr() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("test-slurm.job"));
    let result = filesystem.create_file(path);

    assert!(result.is_err());
}

#[test]
fn createfile_nonexisting_isok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let identifier = random::<u16>();
    let path = FileSystemPath::new(format!("file_{}.txt", identifier));
    let result = filesystem.create_file(path);

    assert!(result.is_ok());
}

#[test]
fn delete_existing_isok() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.delete(path, false);

    assert!(result.is_err());
}

#[test]
fn delete_nonexisting_iserr() {
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
fn readfromfile_nonexisting_iserr() {
    let filesystem = common::create_sftp_filesystem().unwrap();

    let path = FileSystemPath::new(String::from("nonexisting.txt"));
    let result = filesystem.read_from_file(path);

    assert!(result.is_err());
}