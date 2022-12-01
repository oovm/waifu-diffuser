use std::{env, path::PathBuf};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test_var() {
    let lib_dir = PathBuf::from(
        env::var("D:/models/static-onnx/target/Release/Release")
            .expect("[ort] system strategy requires ORT_LIB_LOCATION env var to be set"),
    );
    println!("{:?}", lib_dir.join("libonnxruntime_common.a"));
}
