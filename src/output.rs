// src/output.rs
#[cfg(feature = "colors")]
use colored::*;

pub fn success(msg: &str) {
    #[cfg(feature = "colors")]
    println!("{}", msg.bold().green());
    #[cfg(not(feature = "colors"))]
    println!("{}", msg);
}

pub fn error(msg: &str) {
    #[cfg(feature = "colors")]
    eprintln!("{}", msg.bold().red());
    #[cfg(not(feature = "colors"))]
    eprintln!("{}", msg);
}

pub fn warning(msg: &str) {
    #[cfg(feature = "colors")]
    eprintln!("{}", msg.bold().yellow());
    #[cfg(not(feature = "colors"))]
    eprintln!("{}", msg);
}

pub fn info(msg: &str) {
    println!("{}", msg);
}

pub fn debug(msg: &str) {
    #[cfg(feature = "colors")]
    println!("{}", msg.dimmed());
    #[cfg(not(feature = "colors"))]
    println!("{}", msg);
}
