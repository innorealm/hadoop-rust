fn main() -> std::io::Result<()> {
    prost_build::compile_protos(
        &[
            "proto/ClientNamenodeProtocol.proto",
            "proto/HAServiceProtocol.proto",
            "proto/IpcConnectionContext.proto",
            "proto/ProtobufRpcEngine2.proto",
            "proto/RpcHeader.proto",
            "proto/Security.proto",
        ],
        &["proto"],
    )?;
    Ok(())
}
