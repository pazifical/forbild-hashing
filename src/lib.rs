use std::path::PathBuf;

pub mod editing;
pub mod hashmath;
pub mod hash;

pub const SIZE: u32 = 16;

pub fn parse_args_to_paths() -> Vec<PathBuf> {
    let args = std::env::args().skip(1).collect();

    match parse_args(args) {
        Some(p) => p,
        None => {
            eprintln!("ERROR: No image paths were given. Exiting program.");
            std::process::exit(1);
        }
    }
}

fn parse_args(args: Vec<String>) -> Option<Vec<PathBuf>> {
    if args.len() == 0 {
       return None;
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
    Some(paths)
}

#[cfg(test)]
mod lib_testing {
    use super::*;

    #[test]
    fn test_parse_args_success() {
        let args = vec![
            "data/original/2017_China_Chongqing_Boats.jpg".to_string(),
            "data/original/NoImageToBeFoundHere.jpg".to_string(),
        ];

        let option = parse_args(args.clone()).unwrap();

        assert_eq!(option.len(), 1);
        assert_eq!(option[0].to_str().unwrap(), args[0]);
    }

    #[test]
    fn test_parse_args_failure() {
        assert_eq!(
            parse_args(Vec::new()).is_none(),
            true
        );
    }
}