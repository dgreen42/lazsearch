use std::{env, path::PathBuf};

fn main() {
    let search = env::args().nth(1).expect("Enter search");
    let top_dir = env::current_dir().expect("Directory not found");

    quick_search(&top_dir, &search);
}

fn search_dir(dir: &str, search: &str) -> Option<String> {
    if dir.contains(search) {
        Some(dir.to_string())
    } else {
        None
    }
}

fn nothing() {}

fn quick_search(top_dir: &PathBuf, search: &str) {
    //println!("{:?}", top_dir);
    if !top_dir.is_dir() {
        let result = search_dir(top_dir.as_path().to_str().unwrap(), &search);
        match result {
            Some(..) => println!("{:?}", result.unwrap()),
            None => nothing(),
        }
    } else {
        for one_down in top_dir.read_dir().unwrap() {
            let temp_dir = one_down.unwrap();
            let temp_dir2 = temp_dir.path();
            let cur_dir = temp_dir2.to_str().unwrap();
            let result = search_dir(cur_dir, &search);
            match result {
                Some(..) => println!("{:?}", result.unwrap()),
                None => {
                    let new_top = PathBuf::from(cur_dir);
                    quick_search(&new_top, search);
                }
            }
        }
    }
}
