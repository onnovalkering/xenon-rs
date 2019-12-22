mod common;

use xenon_rs::storage::FileSystemPath;

#[test]
fn create_filesystem() {
    // Arrange
    let filesystem = common::create_sftp_filesystem();

    // Write to file
    let path = FileSystemPath::new(String::from("hello.sh"));
    let buffer = String::from("Hello, world!\n").as_bytes().to_vec();

    let result = filesystem.write_to_file(buffer, path);

    assert!(result.is_ok());
}
