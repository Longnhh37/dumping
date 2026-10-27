// build.rs runs before the project is compiled.
// It reads the .proto file and generates Rust code (structs, server traits, and clients).
fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_prost_build::compile_protos("proto/grpc.proto")?;
    Ok(())
}
