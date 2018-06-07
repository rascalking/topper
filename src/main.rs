extern crate libc;
extern crate regex;

use std::ffi::CStr;
use std::fs;
use std::io;
use std::io::BufRead;
use std::path::Path;
//use std::slice;

use regex::Regex;


// TODO: functions
// TODO: actual error handling
// NOTE: libc functions are flagged unsafe and i have no idea what that
//       means in rust yet


fn main() -> Result<(), io::Error> {
    let re = Regex::new(r"^\d+$").unwrap();
    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() && re.is_match(entry.file_name().to_str().unwrap()) {
            let uid_path = path.join("loginuid");
            let uid_f = fs::File::open(&uid_path)?;
            let mut uid_reader = io::BufReader::new(uid_f);
            let mut line = String::new();
            let line_len = uid_reader.read_line(&mut line)?;

            let mut username: CStr;
            unsafe {
                let uid = line.trim().parse::<u32>().unwrap();
                // TODO: allocate my own memory and use getpwuid_r
                let passwd = libc::getpwuid(uid);
                let name_len = libc::strnlen((*passwd).pw_name, line_len);
                username = CStr::from_ptr((*passwd).pw_name);
                //username = slice::from_raw_parts((*passwd).pw_name, name_len);
            }
            
            println!("{:?} {:?}", path.to_str().unwrap(), username.to_str().unwrap());
        }
    }

    Ok(())
}
