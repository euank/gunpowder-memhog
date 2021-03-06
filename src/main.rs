extern crate argparse;
extern crate libc;
extern crate time;

use argparse::{ArgumentParser, Store};
use std::collections::HashMap;
use std::fmt;
use std::iter::Iterator;
use std::str::FromStr;
use std::thread;
use std::thread::sleep;
use std::time::Duration;
use std::vec::Vec;

/// The `Bytes` type represents a number of bytes.
#[derive(Debug)]
struct Bytes(i64);

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    UnrecognizedPrefix,
    UnparseableInt,
}
impl Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnrecognizedPrefix => "Unrecognized byte prefix",
            Error::UnparseableInt => "Unparseable byte size",
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
        for (i, size) in vec!["b", "kb", "mb", "gb", "tb", "pb", "eb"]
            .into_iter()
            .enumerate()
        {
            byte_size_map.insert(size.to_owned(), 1024i64.pow(i as u32));
        }
        // Larger sizes don't fit in i64

        // Split something of the format "1234MB" into "123", "MB", lowercase the postfix, and then
        // lookup in the map.
        // Note, this means e.g. '1MB2' will be equivilant to '12MB' for our parsing.
        let (nums, chars): (Vec<char>, Vec<char>) = s.chars().partition(|&c| c >= '0' && c <= '9');
        if nums.len() == 0 {
            return Err(Error::UnparseableInt);
        }

        // Normalize postfix string into lowercase with the 'b' postfix, so e.g. '100M' -> m
        let mut postfix_str: String = chars
            .into_iter()
            .map(|c| c.to_lowercase().next().unwrap())
            .collect();
        if postfix_str.len() == 1 && postfix_str.chars().next().unwrap() != 'b' {
            postfix_str.push('b');
        }
        let mut multiplier: i64 = 1;
        if postfix_str.len() > 0 {
            match byte_size_map.get(&postfix_str) {
                Some(num) => multiplier = *num,
                None => return Err(Error::UnrecognizedPrefix),
            }
        }

        let tmpstr: String = nums.into_iter().collect();
        let parsed: Result<i64, std::num::ParseIntError> =
            std::str::FromStr::from_str(tmpstr.as_ref());
        match parsed {
            Ok(x) => Ok(Bytes(x * multiplier)),
            _ => Err(Error::UnparseableInt),
        }
    }
}

fn waste_memory(size: usize) {
    let mut v: Vec<u8> = Vec::with_capacity(size);
    unsafe {
        // Same as pushing 'size' 1s onto it, but much faster. Still fairly slow sadly
        std::ptr::write_bytes(v.as_mut_ptr(), 1u8, size);
    }
    // Forgetting it forever isn't unsafe :D
    std::mem::forget(v);
}

fn main() {
    let mut final_memory = Bytes(10 * 1024 * 1024);
    let mut start_memory = Bytes(0);
    let mut step_duration = 0f64;
    let mut exit_timeout = 0u64;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Take up memory");
        ap.refer(&mut final_memory).add_argument(
            "final memory",
            Store,
            "Memory to reach after timeout",
        );
        ap.refer(&mut start_memory)
            .add_option(&["-m", "--start-memory"], Store, "Starting memory");
        ap.refer(&mut exit_timeout).add_option(
            &["-e", "--exit-timeout"],
            Store,
            "How long to run until exiting in seconds (0 for forever)",
        );
        ap.refer(&mut step_duration).add_option(
            &["-t", "--step-timeout"],
            Store,
            "How long to take to increase memory for from initial -> final in seconds",
        );
        ap.parse_args_or_exit();
    }

    if exit_timeout > 0 {
        thread::spawn(move || {
            sleep(Duration::from_secs(exit_timeout));
            std::process::exit(0);
        });
    }

    waste_memory(start_memory.0 as usize);

    if step_duration > 0f64 {
        for _ in 0..((step_duration * 2f64) as i64) {
            waste_memory(
                ((final_memory.0 - start_memory.0) as f64 / (step_duration * 2f64)) as usize,
            );
            sleep(Duration::from_millis(500));
        }
    } else {
        waste_memory((final_memory.0 - start_memory.0) as usize);
    }

    println!("Done wasting memory");
    loop {
        sleep(Duration::from_millis(500));
    }
}
