fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .compile(&["proto/api.proto"], &["proto"])?;
    println!("cargo:rerun-if-changed=proto/api.proto");
    Ok(())
}
