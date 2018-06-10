extern crate libc;
extern crate regex;

use std::collections::HashMap;
use std::ffi::CStr;
use std::fs;
use std::io;
use std::io::BufRead;
use std::os::raw::c_char;
use std::path::Path;

use regex::Regex;


// TODO: functions
// TODO: actual error handling
// NOTE: libc functions are flagged unsafe and i have no idea what that
//       means in rust yet


fn main() -> Result<(), io::Error> {
    let field_split_re = Regex::new(r":\s+").unwrap();
    let digits_re = Regex::new(r"^\d+$").unwrap();
    let proc_path = Path::new("/proc");

    for entry in fs::read_dir(proc_path)? {
        let entry = entry?;
        let path = entry.path();
        let mut map: HashMap<String, String> = HashMap::new();

        // TODO: collect data into {pid: {field: value, ...}} struct, print later
        if path.is_dir() && 
                digits_re.is_match(entry.file_name().to_str().unwrap()) {
            let status_path = path.join("status");
            let f = fs::File::open(&status_path)?;
            let mut status_reader = io::BufReader::new(f);
            for line in status_reader.lines() {
                let line = line.unwrap();
                let fields: Vec<&str> = field_split_re.splitn(line.trim(), 2).collect();
                map.insert(fields[0].to_string(), fields[1].to_string());
            }
            println!("{} {}",
                     entry.file_name().to_str().unwrap(),
                     map.get("Name").unwrap());
        }
    }

    Ok(())
}

fn uid_to_username(uid: u32) -> String {
    let username: String;

    // TODO: allocate my own memory and use getpwuid_r
    let passwd = unsafe { libc::getpwuid(uid) };
    if passwd.is_null() {
        username = String::from("unknown");
    } else {
        username = string_safe(unsafe { (*passwd).pw_name });
    }
    return username;
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
