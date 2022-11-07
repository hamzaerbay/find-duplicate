use fclones::log::Log;
use fclones::config::GroupConfig;
use fclones::path::Path;
use fclones::{group_files, write_report};
use std::io;

fn main() {
    find_duplicates();
}

fn find_duplicates() {
    // https://github.com/pkolaczk/fclones/blob/master/src/group.rs
    let log = Log::new();
    let mut config = GroupConfig::default();
    // input
    println!("Can you put source dir:");
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            println!("input trimmed {}", input.trim().to_string());
            config.paths = vec![Path::from(input.trim().to_string())];
        } 
        Err(e) => {
            println!("oops! something wrong {}", e)}
        }
    // config.paths = vec![Path::from("./files/")];

    // config.paths = vec![Path::from(input)];
    let groups = group_files(&config, &log).unwrap();
    println!("Found {} groups: ", groups.len());
    write_report(&config, &log, &groups).unwrap();
}