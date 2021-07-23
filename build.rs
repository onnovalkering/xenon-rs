use std::io;

fn main() -> io::Result<()> {
    tonic_build::compile_protos("./src/proto/xenon.proto")?;
    Ok(())
}
