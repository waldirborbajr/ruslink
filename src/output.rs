// src/output.rs
use colored::*;

pub fn success(msg: &str) {
    println!("{}", msg.bold().green());
}

pub fn error(msg: &str) {
    eprintln!("{}", msg.bold().red());
}

pub fn warning(msg: &str) {
    eprintln!("{}", msg.bold().yellow());
}

pub fn info(msg: &str) {
    println!("{}", msg);
}

pub fn debug(msg: &str) {
    println!("{}", msg.dimmed());
}
