use std::{fs, io::Write, path::Path};

fn main() {
    println!("cargo:rerun-if-changed=api/service.yml");

    // Generate DTOs (models)
    generate_models();

    // Generate API traits
    generate_api_trait();

    // Organize generated files
    organize_files();

    // Generate `mod.rs` for DTOs
    generate_mod_file("src/apis/models");
}

fn generate_models() {
    let output = std::process::Command::new("npx")
        .args([
            "@openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "api/service.yml",
            "-g",
            "rust",
            "-o",
            "src/apis",
            "--global-property=models",
            "--additional-properties=packageName=apis,modelPackage=apis.dto,sourceFolder="
        ])
        .output()
        .expect("Failed to run OpenAPI generator for DTOs");

    if !output.status.success() {
        panic!(
            "Failed to generate DTOs: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn generate_api_trait() {
    let output = std::process::Command::new("npx")
        .args([
            "@openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "api/service.yml",
            "-g",
            "rust-axum", // Use Rust Axum generator for API traits
            "-o",
            "src/apis",
            "--global-property=apis",
            "--additional-properties=packageName=apis,sourceFolder="
        ])
        .output()
        .expect("Failed to run OpenAPI generator for API traits");

    if !output.status.success() {
        panic!(
            "Failed to generate API traits: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn organize_files() {
    let nested_models = "src/apis/src/models";
    let nested_apis = "src/apis/src/apis"; // Handle misplaced traits
    let destination_models = "src/apis/models";
    let destination_apis = "src/apis";

    // Move models to `dto`
    move_files(nested_models, destination_models);

    // Move API traits to `apis`
    move_files(nested_apis, destination_apis);

    // Remove now-empty directories
    remove_empty_directory(nested_models);
    remove_empty_directory(nested_apis);
    remove_empty_directory("src/apis/src");
}

fn move_files(source: &str, destination: &str) {
    if let Ok(entries) = fs::read_dir(source) {
        for entry in entries {
            if let Ok(entry) = entry {
                let src_path = entry.path();
                if src_path.is_file() {
                    let file_name = entry.file_name();
                    let dest_path = format!("{}/{}", destination, file_name.to_string_lossy());

                    // Move file to the destination folder
                    if let Err(err) = fs::rename(&src_path, &dest_path) {
                        eprintln!("Error moving file {:?}: {}", src_path, err);
                    }
                }
            }
        }
    }
}

fn remove_empty_directory(path: &str) {
    if Path::new(path).exists() {
        if let Err(err) = fs::remove_dir(path) {
            eprintln!("Error removing directory {}: {}", path, err);
        }
    }
}

fn generate_mod_file(destination: &str) {
    let mod_file_path = format!("{}/mod.rs", destination);

    // Read all `.rs` files in the directory
    let mut mod_content = String::new();
    if let Ok(entries) = fs::read_dir(destination) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                // Skip `mod.rs` itself
                if file_name_str != "mod.rs" && file_name_str.ends_with(".rs") {
                    let module_name = file_name_str.strip_suffix(".rs").unwrap();
                    mod_content.push_str(&format!("pub mod {};\n", module_name));
                }
            }
        }
    }

    // Write to `mod.rs`
    if let Err(err) = fs::write(&mod_file_path, mod_content) {
        eprintln!("Error writing mod.rs to {}: {}", mod_file_path, err);
    } else {
        println!("Generated mod.rs at {}", mod_file_path);
    }
}
