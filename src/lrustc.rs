#![experimental]
#![crate_name = "lrustc"]
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

fn main() {
    let args: Vec<String> = os::args();

    let opts = [
        getopts::optopt("d", "dump", "Dump a regular rust file", "PATH"),
        getopts::optopt("o", "", "set output file name", "NAME"),
        getopts::optflag("h", "help", "print this help menu")
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => m,
        Err(f) => fail!(f.to_str()),
    };
    if matches.opt_present("d") {

    }
}
