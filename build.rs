fn build_exchange_grpc() {
    tonic_build::configure()
        .out_dir("src/rpc/exchange")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["proto/exchange/matchengine.proto"],
            &["proto/exchange", "proto/third_party/googleapis"],
        )
        .unwrap();
}

fn build_rollup_grpc() {
    tonic_build::configure()
        .out_dir("src/rpc/rollup")
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(
            &["proto/rollup/rollup.proto"],
            &["proto/rollup", "proto/third_party/googleapis"],
        )
        .unwrap();
}

fn main() {
    build_exchange_grpc();
    build_rollup_grpc();

    // Prevent rebuild if no proto files are changed.
    println!("cargo:rerun-if-changed=proto");
}
