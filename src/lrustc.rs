#![experimental]
#![crate_id = "lrustc"]
#![desc = "lrustc, the Literate Rust extension"]
#![license = "MIT/ASL2"]

#![feature(globs)]
#![feature(phase)]

extern crate getopts;
extern crate regex;
extern crate debug;
#[phase(plugin, link)] extern crate log;
use std::os;

mod liblrustc;

pub fn opts() -> Vec<getopts::OptGroup> {
    use getopts::optopt;
    vec!(optopt("d", "dump", "Dump a regular rust file", "PATH"),)
}

fn main() {
    use getopts::getopts;
    let args: Vec<String> = os::args();
    let matches = match getopts(args.tail(), opts().as_slice()) {
        Ok(m) => { m }
        Err(f) => { fail!(f.to_str()) }
    };
    if matches.opt_present("d") {

    }
}
