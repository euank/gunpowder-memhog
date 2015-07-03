extern crate argparse;
extern crate libc;
extern crate time;

use argparse::{ArgumentParser, Store};
use std::collections::HashMap;
use std::str::FromStr;
use std::thread;
use std::fmt;

/// The `Bytes` type represents a number of bytes.
#[derive(Debug)]
struct Bytes(i64);

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnrecognizedPrefix,
    UnparseableInt
}
impl Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnrecognizedPrefix => "Unrecognized byte prefix",
            Error::UnparseableInt => "Unparseable byte size"
        }
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

/// `FromStr` for `Bytes` allows `Bytes` to be constructed from a string, such as '100MB',
/// correctly.
/// It assumes all postfixes are in binary notation, not metric/decimal. It does not recognize IEC
/// postfixes nor bits.
impl FromStr for Bytes {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut byte_size_map: HashMap<String, i64> = HashMap::new();
        // TODO, pow! macro
        byte_size_map.insert( "b".to_string(), 1);
        byte_size_map.insert("kb".to_string(), 1024);
        byte_size_map.insert("mb".to_string(), 1024 * 1024);
        byte_size_map.insert("gb".to_string(), 1024 * 1024 * 1024);
        byte_size_map.insert("tb".to_string(), 1024 * 1024 * 1024 * 1024);
        byte_size_map.insert("pb".to_string(), 1024 * 1024 * 1024 * 1024 * 1024);
        byte_size_map.insert("eb".to_string(), 1024 * 1024 * 1024 * 1024 * 1024 * 1024);
        // Larger sizes don't fit in i64

        // Split something of the format "1234MB" into "123", "MB", lowercase the postfix, and then
        // lookup in the map.
        // Note, this means e.g. '1MB2' will be equivilant to '12MB' for our parsing.
        let (nums, chars): (Vec<char>, Vec<char>) = s.chars().partition(|&c| c >= '0' && c <= '9');
        if nums.len() == 0 {
            return Err(Error::UnparseableInt);
        }

        // Normalize postfix string into lowercase with the 'b' postfix, so e.g. '100M' -> m
        let mut postfix_str: String = chars.into_iter().map(|c| c.to_lowercase().next().unwrap()).collect();
        if postfix_str.len() == 1  && postfix_str.chars().next().unwrap() != 'b' {
            postfix_str.push('b');
        }
        let mut multiplier: i64 = 1;
        if postfix_str.len() > 0 {
            match byte_size_map.get(&postfix_str) {
                Some(num) => { multiplier = *num },
                None => { return Err(Error::UnrecognizedPrefix) }
            }
        }

        let tmpstr: String = nums.into_iter().collect();
        let parsed: Result<i64, std::num::ParseIntError> = std::str::FromStr::from_str(tmpstr.as_ref());
        match parsed {
            Ok(x) => Ok(Bytes(x * multiplier)),
            _ => Err(Error::UnparseableInt)
        }
    }
}

fn waste_memory(size: usize) {
    println!("Wasting {} bytes", size);
    unsafe {
        let ptr = libc::malloc(size as libc::size_t) as *mut u8;
        std::ptr::write_bytes(ptr as *mut u8, 0, size - 1 as usize);
        // use the memory and then forget about it; if you don't use it then you'll get VIRT memory
        // only
    }
}

fn main() {
    let mut final_memory = Bytes(10 * 1024 * 1024);
    let mut start_memory = Bytes(0);
    let mut step_duration = 0f64;
    let mut exit_timeout = 0f64;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Take up memory");
        ap.refer(&mut final_memory)
            .add_argument("final memory", Store, "Memory to reach after timeout");
        ap.refer(&mut start_memory)
            .add_option(&["-m", "--start-memory"], Store, "Starting memory");
        ap.refer(&mut exit_timeout)
            .add_option(&["-e", "--exit-timeout"], Store, "How long to run until exiting in seconds (0 for forever)");
        ap.refer(&mut step_duration)
            .add_option(&["-t", "--step-timeout"], Store, "How long to take to increase memory for from initial -> final in seconds");
        ap.parse_args_or_exit();
    }

    if exit_timeout > 0f64 {
        thread::spawn(move || {
            std::thread::sleep_ms((exit_timeout * 1000f64) as u32);
            std::process::exit(0);
        });
    }

    waste_memory(start_memory.0 as usize);

    if step_duration > 0f64 {
        for _ in 0..((step_duration*2f64) as i64) {
            waste_memory(((final_memory.0 - start_memory.0) as f64 / (step_duration * 2f64)) as usize);
            std::thread::sleep_ms(500);
        }
    } else {
        waste_memory((final_memory.0 - start_memory.0) as usize);
    }

    loop {
        std::thread::sleep_ms(500);
    }
}
