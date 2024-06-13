use std::process::Command;
use std::path::Path;

fn main() {
    // Rerun npm install / npm run build if the client directory changes
    Command::new("npm").arg("install")
                       .current_dir(Path::new("client"))
                       .status().unwrap();
    Command::new("npm").args(["run", "build"])
                       .current_dir(Path::new("client"))
                       .status().unwrap();

    println!("cargo::rerun-if-changed=client");
}