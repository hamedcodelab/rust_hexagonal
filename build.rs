fn main() {
    println!("cargo:rerun-if-changed=proto");

    tonic_build::configure()
        .build_server(true)
        .build_client(true)
        .file_descriptor_set_path("target/descriptor.bin")
        //.out_dir("gen")
        .compile(
            &["proto/rust_hexagonal/v1/rust_hexagonal.proto"],
            &["proto"],
        )
        .unwrap();
}

