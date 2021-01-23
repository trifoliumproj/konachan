fn main() -> Result<(), Box<dyn std::error::Error>> {
    // tonic_build::compile_protos("definitions/konachan.proto")?;
    tonic_build::configure()
        .build_client(false)
        .out_dir("./src/generated")
        .type_attribute(".konachan.Post", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["definitions/konachan.proto"],
            &["definitions"]
        )?;
    Ok(())
}
