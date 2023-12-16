use std::env::args;
use std::fs::{copy, remove_file, rename};
use std::thread::sleep;
use std::time::Duration;

include!("../../src/windows.rs");

fn main() {
    let mut args = args();
    args.next().unwrap();
    let source = args.next().unwrap();
    let target = args.next().unwrap();
    let runtime = args.next().unwrap();
    let delete = args.next().unwrap() == "1";
    sleep(Duration::from_secs(3));
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
    create_process(&Command::new(source).args(args), &runtime).unwrap();
}
