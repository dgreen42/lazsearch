use std::{
    env,
    error::Error,
    ffi::OsStr,
    fs::{read_dir, DirEntry, ReadDir},
    path::PathBuf,
};

fn main() {
    let search = env::args().nth(1).expect("Enter search");
    let result = String::new();
    let top_dir = env::current_dir().expect("Directory not found");

    for one_down in top_dir.read_dir().unwrap() {
        search_dir(one_down.unwrap().path().to_str().unwrap(), &search);
    }
}

fn search_dir(dir: &str, search: &str) -> Option<String> {
    println!("{:?}", dir);
    if dir.contains(search) {
        Some(dir.to_string())
    } else {
        let path = PathBuf::new();
    }
}
