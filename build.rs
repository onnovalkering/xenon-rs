fn main() -> Result<(), std::io::Error> {
    tonic_build::configure()
        .format(false)
        .compile(&["src/proto/xenon.proto"], &["src/proto"])
}
