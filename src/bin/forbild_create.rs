use forbild_hashing::parse_args_to_paths;
use forbild_hashing::hash::Hash;

#[cfg(not(tarpaulin_include))]
fn main() {
    let paths = parse_args_to_paths();

    for path in &paths {
        let hash = Hash::from_path(path);
        println!("{};{}", path.to_str().unwrap(), hash.to_string());
    }
}