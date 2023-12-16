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
    while let Err(e) = remove_file(&source) {
        match e.kind() {
            ErrorKind::NotFound => break,
            ErrorKind::PermissionDenied => panic!("{}", e),
            _ => continue
        }
    }
    if delete {
        rename(&target, &source).unwrap();
    } else {
        copy(&target, &source).unwrap();
    }
    let mut command = source.clone();
    while let Some(arg) = args.next() {
        command.push_str(" ");
        command.push_str(&arg);
    }
    create_process(&command, &runtime).unwrap();
}
