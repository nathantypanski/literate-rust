#![experimental]
#![crate_name = "lrustc"]
#![desc = "lrustc, the Literate Rust extension"]
#![license = "MIT/ASL2"]

#![feature(globs)]
#![feature(phase)]

extern crate getopts;
extern crate regex;
extern crate debug;
extern crate rustdoc;
extern crate collections;
#[phase(plugin, link)] extern crate log;

use std::os;

mod liblrustc;

fn handle_options(mut args: Vec<String>) -> Option<getopts::Matches> {
    let _binary = args.shift().unwrap();
    let mut opts = vec!(
        getopts::optflag("d", "dump", "Dump a regular rust file"),
        getopts::optopt("o", "", "set output file name", "NAME"),
        getopts::optflag("h", "help", "print this help menu")
    );
    opts.push_all_move(rustdoc::opts());

    let matches = match getopts::getopts(args.as_slice(), opts.as_slice()) {
        Ok(m) => m,
        Err(_) => return None,
    };

    Some(matches)
}

fn main() {
    let args: Vec<String> = os::args();
    let matches = match handle_options(args) {
        Some(matches) => matches,
        None => return
    };
    if matches.opt_present("d") {
        println!("Dumping as Rust");
    }
}
