#![windows_subsystem = "windows"]

use std::env::args;
use std::fs::{copy, remove_file, rename};

include!("../../src/windows.rs");

fn main() {
    let mut args = args();
    args.next().unwrap();
    let source = args.next().unwrap();
    let target = args.next().unwrap();
    let runtime = args.next().unwrap();
    let delete = args.next().unwrap() == "1";
    if let Err(e) = remove_file(&source) {
        if e.kind() != ErrorKind::NotFound {
            panic!("{}", e)
        }
    }
    if delete {
        rename(&target, &source).unwrap();
    } else {
        copy(&target, &source).unwrap();
    }
    create_process(&Command::new(source).args(args), &runtime).unwrap();
}
