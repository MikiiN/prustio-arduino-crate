use std::env;

fn main() {
    // export the path of C++ files (PlatformIO)
    let manifest_dir = match env::var("CARGO_MANIFEST_DIR") {
        Ok(dir) => dir,
        Err(_) => {
            eprintln!("Failed to obtain cargo manifest dir.");
            return;
        }
    };
    let pio_project_path = format!("{}/pio_src", manifest_dir);
    
    println!("cargo:pio_source_path={}", pio_project_path);
}