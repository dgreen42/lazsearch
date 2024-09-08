use colored::Colorize;
use std::{self, env, path::PathBuf};

mod search {

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

fn quick_search(
    imutable_top_dir: &PathBuf,
    top_dir: &PathBuf,
    search: &str,
    option: &str, /*, mut counter: i32*/
) {
    use search::*;
    use std::io::{stdout, Write};
    let top_dir_s = top_dir.as_path().to_str().unwrap().to_string();
    // its not actually imutable but shoudl be treated as such to make -s work properly
    let imutable_top_dir_s = imutable_top_dir.as_path().to_str().unwrap().to_string();
    if !top_dir.is_dir() {
        let searcher = Searcher {
            directory: top_dir_s,
            search: search.to_string(),
        };
        let result = searcher.search();
        match result {
            Some(..) => stdout()
                .write_fmt(format_args!("\n{}", result.unwrap()))
                .unwrap(),
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
            let mut path = String::new();
            match result {
                Some(..) => {
                    if option == "-s" {
                        let imutable_top_dir_vec: Vec<_> =
                            imutable_top_dir_s.split("/").clone().map(|s| s).collect();
                        let top_len = imutable_top_dir_vec.len();
                        let path_split = result.clone().unwrap().to_string();
                        let short_path_vec: Vec<_> =
                            path_split.split("/").clone().map(|s| s).collect();
                        for i in 0..short_path_vec.len() {
                            if i >= top_len {
                                path.push_str(&short_path_vec[i]);
                                if i != short_path_vec.len() - 1 {
                                    path.push_str("/");
                                }
                            }
                        }
                    }
                    if option == "-l" {
                        path.push_str(&cur_dir);
                    }
                    stdout().write_fmt(format_args!("\n{}", path)).unwrap()
                }
                None => {
                    let new_top = PathBuf::from(cur_dir);
                    //counter += 1;
                    //println!("{:.<1$}", "", counter as usize);
                    quick_search(
                        imutable_top_dir,
                        &new_top,
                        search,
                        option, /*,  counter */
                    );
                }
            }
        }
    }
}

fn main() {
    use std::io::{stdout, Write};
    let search_pat = env::args().nth(1).expect("Enter search");
    let option = env::args().nth(2).expect("Please enter option");
    if option == "--help" {
        stdout()
            .write(b"This is the help menu. It will be made soon")
            .unwrap();
    }
    let top_dir = env::current_dir().expect("Directory not found");
    // counter is used as a diagnostic tool to make sure search isn't stalling
    // let mut counter = 0;

    quick_search(&top_dir, &top_dir, &search_pat, &option /*, counter */);
    println!("\n{}", "Done!".green());
}
