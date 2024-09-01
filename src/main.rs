use colored::Colorize;
use std::{env, path::PathBuf, thread};

mod search {
    use std::path::PathBuf;

    pub struct Searcher {
        pub directory: String,
        pub search: String,
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

fn quick_search(top_dir: &PathBuf, search: &str /*, mut counter: i32*/) {
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
                    //counter += 1;
                    //println!("{:.<1$}", "", counter as usize);
                    quick_search(&new_top, search /*,  counter */);
                }
            }
        }
    }
}

fn main() {
    let search_pat = env::args().nth(1).expect("Enter search");
    let top_dir = env::current_dir().expect("Directory not found");
    // counter is used as a diagnostic tool to make sure search isn't stalling
    // let mut counter = 0;

    quick_search(&top_dir, &search_pat /*, counter */);
    println!("{}", "Done!".green());
}
