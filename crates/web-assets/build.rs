use cache_busters::generate_static_files_code;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=images");
    println!("cargo:rerun-if-changed=dist");

    let static_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Example of multiple asset directories
    let asset_dirs = vec![
        PathBuf::from("./js"),
        PathBuf::from("./images"),
        PathBuf::from("./dist"),
    ];  

    let files = vec![];

    generate_static_files_code(&static_out_dir, &asset_dirs, &files).unwrap();
}