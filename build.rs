fn main() {
    tonic_build::compile_protos("proto/solver.proto").unwrap();
}
