extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["proto_src/$param.main_service_name_snake_case$.proto"],
        &[proto_root],
        &proto_root,
        None
    ).expect("Failed to compile gRPC definitions!");
}

