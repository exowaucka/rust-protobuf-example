extern crate protobuf;

mod addressbook;
mod add_person;
mod list_people;

use protobuf::error::ProtobufError;
use std::{env, process};
use std::error::Error;
use std::io::{stderr, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    get_module_name(&args)
        .and_then(|f| get_file_path(&args).and_then(|p| f(p).map_err(From::from)))
        .unwrap_or_else(|e| {
            stderr().write_fmt(format_args!("{}\n", e)).unwrap();
            process::exit(-1);
        })
}

fn get_module_name(args: &Vec<String>)
                   -> Result<fn(&str) -> Result<(), ProtobufError>, Box<Error>> {
    if args.len() >= 2 {
        match args[1].as_ref() {
            "add_person" => Ok(add_person::execute),
            "list_people" => Ok(list_people::execute),
            other => {
                Err(From::from(format!("Unexpected module name: {}. (expected 'add_person' or \
                                        'list_people')",
                                       other)))
            }
        }
    } else {
        Err(From::from("Usage: cargo run <module_name> <file_path>"))
    }
}

fn get_file_path(args: &Vec<String>) -> Result<&str, Box<Error>> {
    if args.len() >= 3 {
        Ok(&args[2])
    } else {
        Err(From::from("Usage: cargo run <module_name> <file_path>"))
    }
}
