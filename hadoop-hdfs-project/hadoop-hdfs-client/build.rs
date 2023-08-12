fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &["proto/ClientNamenodeProtocol.proto"],
        &["proto", "../../hadoop-common-project/hadoop-common/proto"],
    )?;
    Ok(())
}
