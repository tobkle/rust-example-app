use cache_busters::generate_static_files_code;
use std::env;
use std::fs;
use std::path::PathBuf;

fn copy_dir_recursive(src: &PathBuf, dst: &PathBuf) -> std::io::Result<()> {
    if !src.exists() {
        return Ok(());
    }

    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }
    fs::create_dir_all(dst)?;

    let mut stack = vec![src.clone()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let rel = path.strip_prefix(src).unwrap();
            let target = dst.join(rel);

            if path.is_dir() {
                fs::create_dir_all(&target)?;
                stack.push(path);
            } else {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::copy(&path, &target)?;
            }
        }
    }

    Ok(())
}

fn main() {
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=images");
    println!("cargo:rerun-if-changed=dist");
    println!("cargo:rerun-if-changed=../asset-pipeline/dist");

    let static_out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Publish `asset-pipeline` artifacts into this crate's `dist/` so the web-server can
    // serve them under `/dist/...` and they become available via `web_assets::files::*`.
    let asset_pipeline_dist_src = PathBuf::from("../asset-pipeline/dist");
    let asset_pipeline_dist_dst = PathBuf::from("./dist/asset-pipeline");
    copy_dir_recursive(&asset_pipeline_dist_src, &asset_pipeline_dist_dst)
        .expect("Failed to copy asset-pipeline dist into web-assets dist");

    // Example of multiple asset directories
    let asset_dirs = vec![
        PathBuf::from("./js"),
        PathBuf::from("./images"),
        PathBuf::from("./dist"),
    ];  

    let files = vec![];

    generate_static_files_code(&static_out_dir, &asset_dirs, &files).unwrap();
}