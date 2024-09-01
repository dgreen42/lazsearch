use colored::Colorize;
use std::{env, path::PathBuf, thread};

fn main() {
    let search_pat = env::args().nth(1).expect("Enter search");
    let top_dir = env::current_dir().expect("Directory not found");
    quick_search(&top_dir, &search_pat);
    println!("{}", "Done!".green());
}
mod search {
    use std::path::PathBuf;
    pub struct Searcher {
        pub directory: String,
        pub search: String,
    }
    pub enum Recurse {
        Base,
        Continue(Box<Searcher>),
    }
    impl Searcher {
        pub fn search(&self) -> Option<String> {
            if self.directory.contains(&self.search) {
                Some(self.directory.to_string())
            } else {
                None
            }
        }
    }
    pub fn do_nothing() {}
}
fn quick_search(top_dir: &PathBuf, search: &str) {
    use search::*;
    let top_dir_s = top_dir.as_path().to_str().unwrap().to_string();
    if !top_dir.is_dir() {
        let searcher = Searcher {
            directory: top_dir_s,
            search: search.to_string(),
        };
        let result = searcher.search();
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
            let cur_dir = temp_dir2.to_str().unwrap().to_string();
            let searcher = Searcher {
                directory: cur_dir.clone(),
                search: search.to_string(),
            };
            let result = searcher.search();
            match result {
                Some(..) => {
                    println!("{} {:?}", "Match at: ".cyan(), result.unwrap());
                }
                None => {
                    let new_top = PathBuf::from(cur_dir);
                    quick_search(&new_top, search);
                }
            }
        }
    }
}
