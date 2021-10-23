use std::path::PathBuf;

pub mod editing;
pub mod hashmath;
pub mod hash;

pub const SIZE: u32 = 16;

pub fn parse_args_to_paths() -> Vec<PathBuf> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.len() == 0 {
        eprintln!("ERROR: No image paths were given. Exiting program.");
        std::process::exit(1);
    }

    // Converting arguments to Paths and checking their validity
    let mut paths= Vec::new();
    for arg in &args {
        let path = PathBuf::from(arg);
        if path.exists() {
            paths.push(path);
        } else {
            eprintln!("WARNING: No file found with path: {:?}", path);
            eprintln!("         File will be skipped.");
        }
    }
    paths
}
