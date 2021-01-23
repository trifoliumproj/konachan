fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("definitions/konachan.proto")?;
    tonic_build::configure()
        .build_client(false)
        .out_dir("./src/generated")
        .compile(
            &["definitions/konachan.proto"],
            &["definitions"]
        )?;
    Ok(())
}
