use std::env::args;
use std::env::args_os;
use std::file;
use std::fs::File;
use std::io;
use std::process::exit;

pub struct Lox {

}

impl Lox {

  // pub fn run_file(arg){}
  pub fn main_entry() {
    if args().len() > 1 {
      println!("Usage: jlox [script]");
      exit(64);
    }
    if args().len() == 1 {
      // run_file(args());
    }else {
    } 
  }
}
