use core::panic;
use std::env;
use std::fs::{rename, File};
// my attempt to convert new_file.py to a rust exe instead.
fn help(s: &str) {
    println!("I move everything in main.rs to a new file of your making.\nUsage: {} <file_name>. file_name must adhere to windows file naming conventions.", s.to_string());
}
fn new_file(to: &str) {
    let _ = rename("main.rs", {
        match to {
            to if to[to.len()-3..].contains(".rs") => to.to_string(), //if the last 3 letters are .rs
            &_ => format!("{}.rs", to.to_string()),
        }
    });
    let _ = File::create_new("main.rs");
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let self_name = &args[0];
    match args.len() {
        2 => new_file(&args[1]),
        _ => help(&self_name),
    }

    // The first argument is the path that was used to call the program.
    // println!("My path is {}.", args[0]);

    // The rest of the arguments are the passed command line parameters.
    // Call the program like this:
    //   $ ./args arg1 arg2
    // println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
}
