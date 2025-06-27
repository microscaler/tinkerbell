// crates/docsbookgen/src/main.rs
use std::{fs, path::Path, process::Command};

fn main() {
    println!("📘 Running docsbookgen...");

    let mdbook_src = Path::new("docs/mdbook");
    let docbook_out = Path::new("docsbook");

    // Ensure the docsbook output directory exists
    if !docbook_out.exists() {
        fs::create_dir_all(docbook_out).expect("Failed to create docsbook dir");
    }

    // Step 1: Generate Rust documentation via cargo doc
    println!("🦀 Generating Rust API docs...");
    let status = Command::new("cargo")
        .args(["doc", "--workspace", "--no-deps"])
        .status()
        .expect("Failed to run cargo doc");
    assert!(status.success(), "cargo doc failed");

    // Step 2: Copy cargo doc to docsbook/api
    let api_doc_path = docbook_out.join("api");
    if api_doc_path.exists() {
        fs::remove_dir_all(&api_doc_path).expect("Failed to clear old API docs");
    }
    fs::create_dir_all(&api_doc_path).unwrap();
    fs_extra::dir::copy("target/doc", &api_doc_path, &fs_extra::dir::CopyOptions::new().overwrite(true).copy_inside(true))
        .expect("Failed to copy API docs");

    // Step 3: Build mdbook (if exists)
    if mdbook_src.exists() {
        println!("📚 Building mdbook...");
        let status = Command::new("mdbook")
            .arg("build")
            .arg(mdbook_src)
            .status()
            .expect("Failed to build mdbook");
        assert!(status.success(), "mdbook build failed");

        // Step 4: Copy mdbook output to docsbook/md
        let mdbook_output_path = mdbook_src.join("book");
        let dest = docbook_out.join("md");

        if dest.exists() {
            fs::remove_dir_all(&dest).expect("Failed to clear old mdbook");
        }
        fs::create_dir_all(&dest).unwrap();
        fs_extra::dir::copy(&mdbook_output_path, &dest, &fs_extra::dir::CopyOptions::new().overwrite(true).copy_inside(true))
            .expect("Failed to copy mdbook docs");
    } else {
        println!("⚠️  No docs/mdbook/ folder found — skipping mdbook build");
    }

    println!("✅ docsbookgen completed. Docs available in ./docsbook/");
}
