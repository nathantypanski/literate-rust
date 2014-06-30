// Copyright 2012-2014 The Literate Rust Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![experimental]
#![crate_id = "lrustc"]
#![desc = "lrustc, Literate Rust"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![feature(globs)]
#![feature(phase)]

extern crate regex;
extern crate debug;
#[phase(plugin, link)] extern crate log;

use std::io::{File, fs, Open, Read};
use std::path::posix::Path;

/// Convert a lrs path to a rs path.
fn lrs_path_to_rs(path: Path) -> Option<Path> {
    path.extension_str().and_then(
        |ext| match ext {
            "lrs" => {
                let mut newpath = path.clone();
                newpath.set_extension("rs");
                Some(newpath)
            }
            _ => { None }
        }
    )
}

fn dump(path: Path) {
    let file = match File::open_mode(&path, Open, Read) {
        Ok(f) => f,
        Err(e) => fail!("file error: {}", e),
    };

}

#[cfg(test)]
mod tests {
    use super::lrs_path_to_rs;

    fn expect_path(path: &str, expect: &str) {
        let path = Path::new(path.clone());
        let ps = lrs_path_to_rs(path).unwrap();
        assert!(ps.as_str().unwrap() == expect);
    }

    #[test]
    fn test_lrs_path_to_rs() {
        expect_path("/tmp/lrustc.lrs", "/tmp/lrustc.rs");
        expect_path("~/tmp/afs.lrs", "~/tmp/afs.rs");
        assert!(lrs_path_to_rs(Path::new("/tmp/lr.rs")).is_none());
        assert!(lrs_path_to_rs(Path::new("lr.rs")).is_none());
    }

}
