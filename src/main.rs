extern crate regex;

use std::fs;
use std::io;
use std::path::Path;

use regex::Regex;


fn main() -> Result<(), io::Error> {
    let re = Regex::new(r"^\d+$").unwrap();
    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() && re.is_match(entry.file_name().to_str().unwrap()) {
            println!("{:#?}", path);
        }
    }

    Ok(())
}
