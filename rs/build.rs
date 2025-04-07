use std::fs;
use std::process::Command;

fn main() {
    if std::env::var("DEVOPS_BUILD").unwrap_or_default() == "1" {
        Command::new("git")
            .args(["fetch", "--tags", "--force"])
            .output()
            .expect("Failed to pull latest tags");
        // Get the latest Git tag matching "v*"
        let output = Command::new("git")
            .args(["describe", "--tags", "--match", "v*", "--abbrev=0"])
            .output()
            .expect("Failed to get latest Git tag");

        if !output.status.success() {
            panic!("Error retrieving Git tag");
        }

        let version = String::from_utf8(output.stdout)
            .unwrap()
            .trim()
            .replace("v", "");

        // Set as environment variable for Rust
        println!("cargo:rustc-env=GIT_VERSION={}", version);

        // Overwrite version in Cargo.toml (Optional)
        let cargo_toml_path = "Cargo.toml";
        let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
        let updated_cargo_toml = cargo_toml
            .lines()
            .map(|line| {
                if line.starts_with("version =") {
                    format!("version = \"{}\"", version)
                } else {
                    line.to_string()
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        fs::write(cargo_toml_path, updated_cargo_toml).expect("Failed to update Cargo.toml");

        println!("Updated Cargo.toml version to {}", version);
    }
}
