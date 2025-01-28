use std::{
    fs,
    io::{self, Write},
    path::Path,
    process::Command,
};

fn main() {
    println!("cargo:rerun-if-changed=api/service.yml");

    // Ensure required directories exist
    ensure_directories_exist();

    // Clean up previous generated files
    clean_generated_files();

    // Generate DTOs (models)
    generate_models();

    // Generate API traits
    generate_api_trait();

    // Generate `mod.rs` for DTOs
    generate_mod_file("generated/src/models");

    // Generate `mod.rs` for APIs
    generate_mod_file("generated/src/apis");

    // Fix imports and add `ToSchema`
    fix_generated_models("generated/src/models");
}

/// Ensures the necessary directories exist
fn ensure_directories_exist() {
    let required_dirs = vec!["generated/models", "generated/apis", "generated/docs"];

    for dir in &required_dirs {
        if !Path::new(dir).exists() {
            fs::create_dir_all(dir).expect(&format!("Failed to create directory: {}", dir));
            println!("Created directory: {}", dir);
        }
    }
}

/// Removes previously generated files to avoid conflicts
fn clean_generated_files() {
    let paths_to_clean = vec!["generated/models", "generated/apis", "generated/docs"];

    for path in &paths_to_clean {
        if Path::new(path).exists() {
            fs::remove_dir_all(path).expect(&format!("Failed to remove directory: {}", path));
            println!("Removed directory: {}", path);
        }
    }
}

fn generate_models() {
    let output = Command::new("npx")
        .args([
            "@openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "api/service.yml",
            "-g",
            "rust",
            "-o",
            "generated",
            "--global-property=models",
            "--additional-properties=packageName=apis,modelPackage=apis.dto,sourceFolder=",
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
    let output = Command::new("npx")
        .args([
            "@openapitools/openapi-generator-cli",
            "generate",
            "-i",
            "api/service.yml",
            "-g",
            "rust-axum",
            "-o",
            "generated",
            "--global-property=apis",
            "--additional-properties=packageName=apis,sourceFolder=",
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

fn generate_mod_file(destination: &str) {
    let mod_file_path = format!("{}/mod.rs", destination);

    let mut mod_content = String::new();
    if let Ok(entries) = fs::read_dir(destination) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str != "mod.rs" && file_name_str.ends_with(".rs") {
                    let module_name = file_name_str.strip_suffix(".rs").unwrap();
                    mod_content.push_str(&format!("pub mod {};\n", module_name));
                }
            }
        }
    }

    if let Err(err) = fs::write(&mod_file_path, mod_content) {
        eprintln!("Error writing mod.rs to {}: {}", mod_file_path, err);
    } else {
        println!("Generated mod.rs at {}", mod_file_path);
    }
}

/// **Fixes generated model files:**
/// 1. Adds `use utoipa::ToSchema;`
/// 2. Adds `#[derive(ToSchema)]` to structs
fn fix_generated_models(models_dir: &str) {
    if let Ok(entries) = fs::read_dir(models_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() && path.extension().map_or(false, |ext| ext == "rs") {
                    if let Ok(mut content) = fs::read_to_string(&path) {
                        // Ensure `use utoipa::ToSchema;` is present
                        if !content.contains("use utoipa::ToSchema;") {
                            content = format!("use utoipa::ToSchema;\n{}", content);
                        }

                        // Add `ToSchema` to struct definitions
                        content = content
                            .replace("#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]",
                                     "#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize, ToSchema)]");

                        if let Err(err) = fs::write(&path, content) {
                            eprintln!("Error writing file {:?}: {}", path, err);
                        } else {
                            println!("Updated {:?}", path);
                        }
                    }
                }
            }
        }
    }
}
