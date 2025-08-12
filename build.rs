fn main() -> Result<(), std::io::Error> {
    std::fs::create_dir_all("src/genpb")?;
    tonic_prost_build::configure()
        .out_dir("src/genpb")
        .build_server(false)
        .codec_path("tonic_prost::ProstCodec")        
        .compile_protos(&["proto/defs/cerbos/svc/v1/svc.proto"], &["proto/defs/"])
}
