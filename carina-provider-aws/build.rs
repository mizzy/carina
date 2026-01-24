use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=../schemas");

    let schema_dir = Path::new("../schemas");
    let out_dir = Path::new("src/generated");

    // Create output directory if it doesn't exist
    fs::create_dir_all(out_dir).expect("Failed to create generated directory");

    // Generate mod.rs for the generated module
    let mut mod_content = String::new();
    mod_content.push_str("// Auto-generated module - do not edit\n\n");

    // Process each schema file
    if let Ok(entries) = fs::read_dir(schema_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "json")
                && let Some(stem) = path.file_stem()
            {
                let module_name = stem.to_string_lossy().to_lowercase();

                match generate_types_from_schema(&path, out_dir, &module_name) {
                    Ok(()) => {
                        mod_content.push_str(&format!("pub mod {};\n", module_name));
                    }
                    Err(e) => {
                        eprintln!("Warning: Failed to generate types for {:?}: {}", path, e);
                    }
                }
            }
        }
    }

    // Write mod.rs
    let mod_path = out_dir.join("mod.rs");
    fs::write(&mod_path, mod_content).expect("Failed to write mod.rs");
}

fn generate_types_from_schema(
    schema_path: &Path,
    out_dir: &Path,
    module_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema_content = fs::read_to_string(schema_path)?;
    let schema: serde_json::Value = serde_json::from_str(&schema_content)?;

    // Use typify to generate Rust types from the JSON schema
    let mut type_space =
        typify::TypeSpace::new(typify::TypeSpaceSettings::default().with_struct_builder(true));

    type_space.add_root_schema(serde_json::from_value(schema)?)?;

    let tokens = type_space.to_stream();
    let code = prettyplease_maybe(tokens.to_string());

    // Add necessary imports at the top
    let full_code = format!(
        r#"// Auto-generated from {} - do not edit
#![allow(dead_code, unused_imports, clippy::all)]

use serde::{{Deserialize, Serialize}};

{}
"#,
        schema_path.display(),
        code
    );

    let out_path = out_dir.join(format!("{}.rs", module_name));
    fs::write(&out_path, full_code)?;

    Ok(())
}

fn prettyplease_maybe(code: String) -> String {
    // Try to format with prettyplease if available, otherwise return as-is
    match syn::parse_file(&code) {
        Ok(syntax_tree) => prettyplease::unparse(&syntax_tree),
        Err(_) => code,
    }
}
