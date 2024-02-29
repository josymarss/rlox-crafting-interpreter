use std::env::args;
use std::env::args_os;
use std::file;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::process::exit;

pub struct Lox {

}

impl Lox {

  pub fn run_file(arg:Option<String>) {
    let bytes = BufReader::new(File::open(arg()?));
    run(bytes, )
  }

  pub fn run_prompt() {}

  pub fn run() {

  }

  pub fn main_entry() {
    if args().len() > 1 {
      println!("Usage: jlox [script]");
      exit(64);
    }
    if args().len() == 1 {
      Lox::run_file(args().nth(0));
    }else {
      Lox::run_prompt();
    } 
  }
}
