extern crate libc;
extern crate regex;

use std::ffi::CStr;
use std::fs;
use std::io;
use std::io::BufRead;
use std::os::raw::c_char;
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
            uid_reader.read_line(&mut line)?;
            let uid = line.trim().parse::<u32>().unwrap();

            let mut username: String;
            unsafe {
                // TODO: allocate my own memory and use getpwuid_r
                let passwd = libc::getpwuid(uid);
                if passwd.is_null() {
                    username = String::from("unknown");
                } else {
                    username = string_safe((*passwd).pw_name);
                }
            }
            
            println!("{:?} {:?}", path.to_str().unwrap(), username);
        }
    }

    Ok(())
}

fn string_safe(c_string: *const c_char) -> String {
    // TODO: probably want to wrap the String in an error instead of returning empty string
    if c_string.is_null() {
        return String::from("");
    }
    else {
        unsafe {
            return CStr::from_ptr(c_string).to_string_lossy().into_owned();
        }
    }
}
