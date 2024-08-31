use colored::Colorize;
use std::{env, path::PathBuf};

fn main() {
    let search_pat = env::args().nth(1).expect("Enter search");
    let top_dir = env::current_dir().expect("Directory not found");

    quick_search(&top_dir, &search_pat);
    println!("{}", "Done!".green());
}

mod search {
    pub fn search_dir(dir: &str, search: &str) -> Option<String> {
        if dir.contains(search) {
            Some(dir.to_string())
        } else {
            None
        }
    }
    // do absolutely nothing cause why would you want to print something if there is no match?
    pub fn do_nothing() {}
}

fn quick_search(top_dir: &PathBuf, search: &str) {
    use search::{do_nothing, search_dir};
    if !top_dir.is_dir() {
        let result = search_dir(top_dir.as_path().to_str().unwrap(), &search);
        match result {
            Some(..) => println!("{} {:?}", "Match at: ".cyan(), result.unwrap()),
            None => do_nothing(),
        }
    } else {
        for one_down in top_dir.read_dir().unwrap() {
            let temp_dir = match one_down {
                Ok(dir) => dir,
                Err(err) => panic!("fuck: {:?}", err),
            };
            let temp_dir2 = temp_dir.path();
            let cur_dir = temp_dir2.to_str().unwrap();
            let result = search_dir(cur_dir, &search);
            match result {
                Some(..) => {
                    println!("{} {:?}", "Match at: ".cyan(), result.unwrap());
                    let new_top = PathBuf::from(cur_dir);
                    quick_search(&new_top, search);
                }
                None => {
                    let new_top = PathBuf::from(cur_dir);
                    quick_search(&new_top, search);
                }
            }
        }
    }
}
