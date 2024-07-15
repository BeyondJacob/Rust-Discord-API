use std::env;
use std::fs;
use std::path::Path;

/// Entry point of the build script. Generates `commands_mod.rs` based on the `commands` directory
/// created by the X user for project.
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("commands_mod.rs");

    let commands_dir = Path::new("src/commands");
    if commands_dir.exists() {
        let mut mod_rs_content = String::new();
        mod_rs_content.push_str("pub fn register_commands(router: &mut crate::CommandRouter) {\n");
        generate_mod_rs(commands_dir, &mut mod_rs_content, "");
        mod_rs_content.push_str("}\n");

        // Write the content to commands_mod.rs
        fs::write(dest_path, mod_rs_content).unwrap();
    }
}

/// Recursively generates `mod.rs` content for the given directory.
///
/// # Arguments
///
/// * `path` - The path of the directory to process.
/// * `mod_rs_content` - The string to append the generated content to.
/// * `parent_mod` - The parent module path, used for nested modules.
fn generate_mod_rs(path: &Path, mod_rs_content: &mut String, parent_mod: &str) {
    if path.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            let file_name = path.file_name().unwrap().to_str().unwrap();

            if path.is_dir() {
                let sub_mod_name = file_name;
                let full_mod_name = if parent_mod.is_empty() {
                    sub_mod_name.to_string()
                } else {
                    format!("{}::{}", parent_mod, sub_mod_name)
                };
                mod_rs_content.push_str(&format!("pub mod {} {{\n", sub_mod_name));
                generate_mod_rs(&path, mod_rs_content, &full_mod_name);
                mod_rs_content.push_str("}\n");
            } else if file_name.ends_with(".rs") && file_name != "mod.rs" {
                let module_name = file_name.strip_suffix(".rs").unwrap();
                let full_mod_name = if parent_mod.is_empty() {
                    module_name.to_string()
                } else {
                    format!("{}::{}", parent_mod, module_name)
                };
                mod_rs_content.push_str(&format!("pub mod {};\n", module_name));
                mod_rs_content.push_str(&format!(
                    "router.register_command(\"!{}\", Arc::new({}::{}));\n",
                    module_name, full_mod_name, capitalize_first(module_name)
                ));
            }
        }
    }
}

/// Capitalizes the first letter of the given string.
///
/// # Arguments
///
/// * `s` - The string to capitalize.
///
/// # Returns
///
/// The capitalized string.
fn capitalize_first(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
