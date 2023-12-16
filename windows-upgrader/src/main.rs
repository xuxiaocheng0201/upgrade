use std::env::args;
use std::fs::{remove_file, rename};
use std::path::PathBuf;

include!("../../src/windows.rs");

fn main() {
    let mut args = args();
    let source = args.next().unwrap();
    let target = args.next().unwrap();
    while let Err(e) = remove_file(&source) {
        match e.kind() {
            ErrorKind::NotFound => break,
            ErrorKind::PermissionDenied => panic!("{}", e),
            _ => continue
        }
    }
    rename(&target, &source).unwrap();
    let mut command = target.clone();
    while let Some(arg) = args.next() {
        command.push_str(" ").push_str(&arg);
    }
    let mut runtime = PathBuf::from(target);
    runtime.pop();
    create_process(&command, runtime.to_str().unwrap()).unwrap();
}
