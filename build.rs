fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("gauge-proto/helloworld.proto")?;
    tonic_build::compile_protos("gauge-proto/services.proto")?;
    // tonic_build::configure()
    // .build_server(false)
    // .compile(
    //     &["helloworld.proto"],
    //     &["gauge-proto/"]
    // )?;
    Ok(())
}