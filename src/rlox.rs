use core::panic;
use std::env::args;
use std::env::args_os;
use std::fs::File;
use std::io;
use std::io::BufReader;
use std::io::Read;
use std::process::exit;


// My imports 

mod token;
pub struct Lox {}

impl Lox {
    pub fn run_file(arg: String) {
      let file = File::open(arg);
      match file {
        Ok(file) => {
          let mut bytes = BufReader::new(file);
          let mut buff = String::new();
          let result = bytes.read_to_string(&mut buff);

          Self::run(&mut buff);
        },
        Err(err) => println!("{}", err),
      }
        // 
        // print!("{:?}",file);
    }

    pub fn run_prompt() {
      let mut line: String = String::new(); 
      let result = io::stdin().read_line(&mut line);

      // let buff = BufReader::new(line);

      loop { 
        // let buf = BufReader::new(line);
        match result {
          Ok(value) => println!("{}", line),
          Error => panic!("Something wrong!"),
        }
        
        Self::run(&line);
      }
        
    }

    pub fn run(source: &String) {
      let mut reader;
      let tokens = io::stdin().read_line(&mut reader);
    
      // For now, just print the tokens.
      for token in tokens.iter() {
        println!("{}",token);
      }
         
    }

    pub fn main_entry() {
        if args_os().len() > 1 {
            println!("Usage: jlox [script]");
            exit(64);
        }
        if args_os().len() == 1 {
            let path = args().nth(0).expect("Some error occured!");
            Lox::run_file(path);
        } else {
            Lox::run_prompt();
        }
    }
  }
